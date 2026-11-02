#include <lean/lean.h>
extern lean_object *initialize_Init(uint8_t);
extern void lean_initialize_runtime_module(void);
#ifdef USE_ZR
extern size_t zr_low_string_intercalate(size_t,size_t);
extern size_t zr_low_int_repr(size_t);
extern uint32_t zr_low_string_utf8_get_fast(size_t,size_t);
extern size_t zr_low_string_utf8_next_fast(size_t,size_t);
#else
extern lean_object *l_String_intercalate(lean_object*,lean_object*);
extern lean_object *l_Int_repr(lean_object*);
#endif
void test_init(void) { lean_initialize_runtime_module(); lean_dec(initialize_Init(1)); }
void test_retain(lean_object *o) { lean_inc(o); }
void test_release(lean_object *o) { lean_dec(o); }
lean_object *test_text(const char *s,size_t len) { return lean_mk_string_from_bytes(s,len); }
const char *test_data(lean_object *s) { return lean_string_cstr(s); }
size_t test_size(lean_object *s) { return lean_string_size(s)-1; }
lean_object *test_cons(lean_object *head,lean_object *tail) {
  lean_object *o=lean_alloc_ctor(1,2,0); lean_ctor_set(o,0,head); lean_ctor_set(o,1,tail); return o;
}
lean_object *test_int(const char *digits,uint8_t negative) {
  lean_object *n=lean_cstr_to_nat(digits);
  return negative ? lean_int_neg_succ_of_nat(n) : lean_nat_to_int(n);
}
lean_object *test_join(lean_object *sep,lean_object *xs) {
#ifdef USE_ZR
  return (lean_object*)zr_low_string_intercalate((size_t)sep,(size_t)xs);
#else
  return l_String_intercalate(sep,xs);
#endif
}
lean_object *test_repr(lean_object *n) {
#ifdef USE_ZR
  return (lean_object*)zr_low_int_repr((size_t)n);
#else
  return l_Int_repr(n);
#endif
}
uint32_t test_get(lean_object *s,size_t i) {
#ifdef USE_ZR
  return zr_low_string_utf8_get_fast((size_t)s,(size_t)lean_box(i));
#else
  return lean_string_utf8_get_fast(s,lean_box(i));
#endif
}
size_t test_next(lean_object *s,size_t i) {
#ifdef USE_ZR
  return lean_unbox((lean_object*)zr_low_string_utf8_next_fast((size_t)s,(size_t)lean_box(i)));
#else
  return lean_unbox(lean_string_utf8_next_fast(s,lean_box(i)));
#endif
}
