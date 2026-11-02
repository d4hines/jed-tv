// C ABI over CityJsb/Plant.lean; no `main` here.
#include <lean/lean.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include "cityjsb.h"

#define API __attribute__((visibility("default")))

extern void lean_initialize_runtime_module(void);
extern void lean_initialize_thread(void);
extern void lean_finalize_thread(void);
lean_object *initialize_vibecheck_CityJsb_Plant(uint8_t builtin);
lean_object *cityjsb_init(double h, double v, double gam, double psi, double phi, double alpha);
lean_object *cityjsb_step_lean(lean_obj_arg s, double elev, double ail, double rud, double thr);

struct cityjsb { lean_object *st; };

static pthread_once_t init_once = PTHREAD_ONCE_INIT;
static int init_ok;
static pthread_t init_thread;
static pthread_key_t thread_key;
static _Thread_local int thread_ready;

static void thread_exit(void *p) { (void)p; lean_finalize_thread(); }

static void runtime_init(void) {
  lean_initialize_runtime_module();
  lean_object *res = initialize_vibecheck_CityJsb_Plant(1);
  lean_io_mark_end_initialization();
  if (!lean_io_result_is_ok(res)) { lean_io_result_show_error(res); lean_dec_ref(res); return; }
  lean_dec_ref(res);
  pthread_key_create(&thread_key, thread_exit);
  init_thread = pthread_self();
  thread_ready = 1;
  init_ok = 1;
}

// Lean needs per-thread allocator state on threads it did not create.
static int ensure_runtime(void) {
  pthread_once(&init_once, runtime_init);
  if (!init_ok) return 0;
  if (!thread_ready) {
    if (!pthread_equal(pthread_self(), init_thread)) {
      lean_initialize_thread();
      pthread_setspecific(thread_key, (void *)1);
    }
    thread_ready = 1;
  }
  return 1;
}

API cityjsb *cityjsb_new(double h_m, double v, double gam, double psi, double phi, double alpha) {
  if (!ensure_runtime()) return NULL;
  cityjsb *p = malloc(sizeof *p);
  if (!p) return NULL;
  p->st = cityjsb_init(h_m, v, gam, psi, phi, alpha);
  return p;
}

API void cityjsb_step(cityjsb *p, double elev, double ail, double rud, double thr, double obs[CITYJSB_NOBS]) {
  ensure_runtime();
  lean_object *res = cityjsb_step_lean(p->st, elev, ail, rud, thr);
  lean_object *st = lean_ctor_get(res, 0);
  lean_object *fa = lean_ctor_get(res, 1);
  lean_inc(st);
  memcpy(obs, lean_float_array_cptr(fa), sizeof(double) * CITYJSB_NOBS);
  lean_dec_ref(res);
  p->st = st;
}

API cityjsb *cityjsb_clone(const cityjsb *p) {
  ensure_runtime();
  cityjsb *c = malloc(sizeof *c);
  if (!c) return NULL;
  lean_inc(p->st);
  c->st = p->st;
  return c;
}

API void cityjsb_free(cityjsb *p) {
  if (!p) return;
  ensure_runtime();
  lean_dec(p->st);
  free(p);
}
