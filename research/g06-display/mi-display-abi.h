/* Reconstructed from EXACT V109 mik.ko + libmi3.so; outer wrapper corrected
 * after the native hardware query exposed the missing ioctl_wrapper layer.
 * These are wire layouts, not complete vendor SDK declarations.
 * Use kernel INI auto-init first. Do not publish daemon-ready to mask failures.
 */
#ifndef G06_V109_MI_DISPLAY_ABI_H
#define G06_V109_MI_DISPLAY_ABI_H
#include <stdint.h>
struct g06_mi_disp_init {
    uint32_t output_timing; /* +0; stock mik_init.ini: 6 */
    uint32_t tv_system;     /* +4; stock: 6 */
    uint32_t de_flicker;    /* +8; stock: 0 */
};
struct g06_mi_osd_init {
    uint8_t wait_sync;       /* +0 */
    uint8_t auto_flip;       /* +1 */
    uint8_t pad0[2];
    uint32_t canvas_mode;    /* +4 */
    uint16_t surface_count;  /* +8; 0 asks for default 64 */
    uint8_t wait_idle;       /* +10 */
    uint8_t pad1;
};
struct g06_mi_module_status {
    uint32_t selector; /* 1 daemon-ready, 6 DISP, 9 OSD, 40 mik load */
    uint8_t ready;
    uint8_t pad[3];
};
/* 16-byte SYS argument, matching libmi3 zero high words.
 * user_address is a pointer to g06_mi_module_status in the caller's address
 * space; on ARM32 set its high word to zero. attr=5 means module status.
 */
struct g06_mi_sys_attr_wire { uint64_t attr; uint64_t user_address; };
/* REQUIRED outer envelope passed to the actual ioctl syscall. Exact libmi3
 * ioctl_wrapper at 0x5090c stores _IOC_SIZE(request) at +0 and the address of
 * its argument at +8. This applies to DISP/OSD too: their 12-byte structs are
 * arguments addressed by this envelope, not direct syscall arguments.
 */
struct g06_mi_ioctl_wire { uint16_t bytes; uint8_t reserved[6]; uint64_t argument; };
#define G06_MI_SYS_GET_ATTR 0xc0101805UL
#define G06_MI_SYS_SET_ATTR 0xc0101806UL
#define G06_MI_DISP_INIT    0xc00c1200UL /* 12-byte argument inside outer wrapper */
#define G06_MI_OSD_INIT     0xc00c1400UL /* 12-byte argument inside outer wrapper */
/* Paths used by exact libmi3.so: /dev/mik!sys, /dev/mik!disp, /dev/mik!osd.
 * Inspect native devtmpfs nodes; vendor naming must not be assumed there.
 * Kernel API equivalents:
 * MI_DISP_Init(&disp), MI_OSD_Init(&osd),
 * MI_SYS_GetAttr(5, &status), MI_SYS_SetAttr(5, &status).
 */
_Static_assert(sizeof(struct g06_mi_disp_init)==12,"DISP layout");
_Static_assert(sizeof(struct g06_mi_osd_init)==12,"OSD layout");
_Static_assert(sizeof(struct g06_mi_module_status)==8,"status layout");
_Static_assert(sizeof(struct g06_mi_sys_attr_wire)==16,"ioctl layout");
_Static_assert(sizeof(struct g06_mi_ioctl_wire)==16,"outer ioctl layout");
#endif
