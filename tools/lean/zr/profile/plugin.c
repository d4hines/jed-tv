/* QEMU TCG plugin for `zr-build profile` (tools/lean/zr/ZrBuild.lean): counts
   executed instructions per translation block and, for the functions named
   by fn=ADDR arguments, calls by return address (and the previous traced
   call's, to see through traced callers); writes them to out=FILE at exit.
   Host code, in C because it is written against QEMU's C plugin API
   (qemu-plugin.h). */
#include <qemu-plugin.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <glib.h>
QEMU_PLUGIN_EXPORT int qemu_plugin_version = QEMU_PLUGIN_VERSION;
typedef struct { uint64_t pc, exec; uint32_t insns; } Ent;
typedef struct { uint64_t fn, lr, prev, n; } Call;
#define M (1u << 16)
static Call calls[M];
static uint64_t traced[64];
static int ntraced;
static struct qemu_plugin_register *lr_reg;
static GByteArray *buf;
static uint32_t last_lr;   /* return address of the previous traced call */
#define N (1u << 21)
static Ent tab[N];
static const char *out = "profile.txt";
static Ent *get(uint64_t pc) {
    uint32_t i = (uint32_t)((pc * 0x9E3779B97F4A7C15ull) >> 43);
    while (tab[i & (N - 1)].pc && tab[i & (N - 1)].pc != pc) i++;
    return &tab[i & (N - 1)];
}
static void exec_cb(unsigned int vcpu, void *e) { (void)vcpu; ((Ent *)e)->exec++; }
static void vcpu_init(unsigned int vcpu, void *u) {
    (void)vcpu; (void)u;
    GArray *regs = qemu_plugin_get_registers();
    for (guint i = 0; i < regs->len; i++) {
        qemu_plugin_reg_descriptor *d = &g_array_index(regs, qemu_plugin_reg_descriptor, i);
        if (!strcmp(d->name, "lr")) lr_reg = d->handle;
    }
    buf = g_byte_array_new();
}
/* entry of a traced function: count (function, return address, previous
   traced call's return address: the caller's caller when the caller is
   itself traced) */
static void call_cb(unsigned int vcpu, void *fn) {
    (void)vcpu;
    uint32_t lr = 0;
    g_byte_array_set_size(buf, 0);
    if (lr_reg && qemu_plugin_read_register(lr_reg, buf) && buf->len >= 4) memcpy(&lr, buf->data, 4);
    uint64_t f = (uint64_t)(uintptr_t)fn;
    uint32_t i = (uint32_t)(((f ^ (lr * 0x9E3779B1ull) ^ (last_lr * 0x85EBCA77ull)) * 0x9E3779B97F4A7C15ull) >> 48);
    while (calls[i & (M - 1)].n &&
           (calls[i & (M - 1)].fn != f || calls[i & (M - 1)].lr != lr || calls[i & (M - 1)].prev != last_lr)) i++;
    Call *c = &calls[i & (M - 1)];
    c->fn = f; c->lr = lr; c->prev = last_lr; c->n++;
    last_lr = lr;
}
static void trans(struct qemu_plugin_tb *tb, void *u) {
    (void)u;
    Ent *e = get(qemu_plugin_tb_vaddr(tb));
    e->pc = qemu_plugin_tb_vaddr(tb);
    e->insns = (uint32_t)qemu_plugin_tb_n_insns(tb);
    qemu_plugin_register_vcpu_tb_exec_cb(tb, exec_cb, QEMU_PLUGIN_CB_NO_REGS, e);
    for (int i = 0; i < ntraced; i++)
        if (traced[i] == e->pc)
            qemu_plugin_register_vcpu_tb_exec_cb(tb, call_cb, QEMU_PLUGIN_CB_R_REGS, (void *)(uintptr_t)e->pc);
}
static void done(void *p) {
    (void)p;
    FILE *f = fopen(out, "w");
    for (uint32_t i = 0; i < N; i++)
        if (tab[i].exec) fprintf(f, "%llx %llu %u\n", (unsigned long long)tab[i].pc,
                                 (unsigned long long)tab[i].exec, tab[i].insns);
    for (uint32_t i = 0; i < M; i++)
        if (calls[i].n) fprintf(f, "c %llx %llx %llx %llu\n", (unsigned long long)calls[i].fn,
                                (unsigned long long)calls[i].lr, (unsigned long long)calls[i].prev,
                                (unsigned long long)calls[i].n);
    fclose(f);
}
QEMU_PLUGIN_EXPORT int qemu_plugin_install(qemu_plugin_id_t id, const qemu_info_t *info, int argc, char **argv) {
    (void)info;
    for (int i = 0; i < argc; i++) {
        if (!strncmp(argv[i], "out=", 4)) out = argv[i] + 4;
        if (!strncmp(argv[i], "fn=", 3) && ntraced < 64) traced[ntraced++] = strtoull(argv[i] + 3, NULL, 16);
    }
    qemu_plugin_register_vcpu_init_cb(id, vcpu_init, NULL);
    qemu_plugin_register_vcpu_tb_trans_cb(id, trans, NULL);
    qemu_plugin_register_atexit_cb(id, done, NULL);
    return 0;
}
