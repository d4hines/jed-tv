/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Experimental G06 boot platform. Kernel bytes are never patched.
 * RIU is explicitly a partial model, not a hardware validation oracle. */
#include "qemu/osdep.h"
#include "qemu/units.h"
#include "qemu/bswap.h"
#include "qapi/error.h"
#include "hw/core/boards.h"
#include "hw/core/sysbus.h"
#include "hw/arm/boot.h"
#include "hw/arm/machines-qom.h"
#include "hw/char/serial-mm.h"
#include "hw/core/qdev-properties.h"
#include "system/system.h"
#include "system/address-spaces.h"
#include "chardev/char.h"
#include "target/arm/cpu.h"
#include "target/arm/gtimer.h"
#include "hw/core/irq.h"
#include "qemu/module.h"
#include "qemu/timer.h"

static uint8_t riu_data[0x800000];
static QEMUTimer *piu_timer;
static qemu_irq piu_irq;
static uint64_t watchdog_clear_count;
static uint64_t watchdog_other_write_count;
static uint64_t watchdog_last_clear_ns;
static void piu_expire(void *opaque)
{
    qemu_set_irq(piu_irq, 1);
    if (riu_data[0x6040] & 1) {
        uint32_t ticks = lduw_le_p(riu_data + 0x6048) | ((uint32_t)lduw_le_p(riu_data + 0x604c) << 16);
        timer_mod(piu_timer, qemu_clock_get_ns(QEMU_CLOCK_VIRTUAL) + MAX(1, muldiv64(ticks, 1000000000, 12000000)));
    }
}
static uint64_t riu_read(void *opaque, hwaddr off, unsigned size)
{
    off += (uintptr_t)opaque;
    if (off == 0x203140) { return 0x1000; } /* MIU drain completed */
    if (off == 0x400200) { return muldiv64(qemu_clock_get_ns(QEMU_CLOCK_VIRTUAL), 12000000, 1000000000); }
    if (off == 0x6090 || off == 0x6094) {
        uint32_t ticks = muldiv64(qemu_clock_get_ns(QEMU_CLOCK_VIRTUAL), 12000000, 1000000000);
        return off == 0x6090 ? (ticks & 0xffff) : (ticks >> 16);
    }
    uint64_t v = 0;
    memcpy(&v, riu_data + off, size);
    return v;
}
static void riu_write(void *opaque, hwaddr off, uint64_t v, unsigned size)
{
    off += (uintptr_t)opaque;
    memcpy(riu_data + off, &v, size);
    if (off == 0x6000) {
        /* Exact V109 HAL_WDT_Clear: one byte, value 1. Physical servicing
         * is proven; expiration is not. Record events without inventing
         * a countdown or treating HAL_WDT_Start as an arming operation. */
        if (size == 1 && v == 1) {
            watchdog_clear_count++;
            watchdog_last_clear_ns = qemu_clock_get_ns(QEMU_CLOCK_VIRTUAL);
        } else {
            watchdog_other_write_count++;
        }
    }
    if (off == 0x604c) { qemu_set_irq(piu_irq, 0); }
    if (off == 0x6040) {
        if ((v & 0x100) && (v & 3)) {
            uint32_t ticks = lduw_le_p(riu_data + 0x6048) | ((uint32_t)lduw_le_p(riu_data + 0x604c) << 16);
            timer_mod(piu_timer, qemu_clock_get_ns(QEMU_CLOCK_VIRTUAL) + MAX(1, muldiv64(ticks, 1000000000, 12000000)));
        } else { timer_del(piu_timer); }
    }
}
static const MemoryRegionOps riu_ops = {
    .read = riu_read, .write = riu_write, .endianness = DEVICE_LITTLE_ENDIAN,
    .valid.min_access_size = 1, .valid.max_access_size = 4,
    .impl.min_access_size = 1, .impl.max_access_size = 4,
};
static void g06_init(MachineState *ms)
{
    MemoryRegion *sys = get_system_memory();
    MemoryRegion *ram = g_new0(MemoryRegion, 1);
    MemoryRegion *firmware = g_new0(MemoryRegion, 1);
    MemoryRegion *reserved = g_new0(MemoryRegion, 1);
    MemoryRegion *bank0 = g_new0(MemoryRegion, 1);
    MemoryRegion *bank1 = g_new0(MemoryRegion, 1);
    MemoryRegion *riu = g_new0(MemoryRegion, 1);
    MemoryRegion *timers = g_new0(MemoryRegion, 1);
    DeviceState *cpus[4];
    int n = ms->smp.cpus;
    object_property_add_uint64_ptr(OBJECT(ms), "watchdog-clear-count",
                                  &watchdog_clear_count, OBJ_PROP_FLAG_READ);
    object_property_add_uint64_ptr(OBJECT(ms), "watchdog-other-write-count",
                                  &watchdog_other_write_count, OBJ_PROP_FLAG_READ);
    object_property_add_uint64_ptr(OBJECT(ms), "watchdog-last-clear-ns",
                                  &watchdog_last_clear_ns, OBJ_PROP_FLAG_READ);
    /* MBOOT/PM51 reserved SRAM/DRAM precedes Linux RAM (PM51_ADDR=20010000). */
    memory_region_init_ram(firmware, NULL, "g06.firmware-ram", 0x200000, &error_fatal);
    memory_region_add_subregion(sys, 0x20000000, firmware);
    memory_region_init_ram(ram, NULL, "g06.ram", 0x5b900000, &error_fatal);
    memory_region_init_alias(bank0, NULL, "g06.bank0", ram, 0, 0x20200000);
    memory_region_init_alias(bank1, NULL, "g06.bank1", ram, 0x20200000, 0x3b700000);
    memory_region_add_subregion(sys, 0x20200000, bank0);
    memory_region_add_subregion(sys, 0x42c00000, bank1);
    /* The gap is excluded from Linux System RAM, not absent DRAM: the stock
     * ION allocator accesses reserved buffers beginning at 0x40400000. */
    memory_region_init_ram(reserved, NULL, "g06.reserved-ram", 0x02800000, &error_fatal);
    memory_region_add_subregion(sys, 0x40400000, reserved);
    memory_region_init_io(riu, NULL, &riu_ops, NULL, "g06.riu.partial", sizeof(riu_data));
    memory_region_add_subregion(sys, 0x1f000000, riu);
    memory_region_init_io(timers, NULL, &riu_ops, (void *)0x400000, "g06.global-timer", 0x1000);
    memory_region_add_subregion(sys, 0x16000000, timers);
    for (int i = 0; i < n; i++) {
        Object *obj = object_new(ARM_CPU_TYPE_NAME("cortex-a55"));
        ARMCPU *cpu = ARM_CPU(obj);
        object_property_set_bool(obj, "aarch64", false, &error_fatal);
        object_property_set_bool(obj, "has_el3", false, &error_fatal);
        object_property_set_bool(obj, "has_el2", false, &error_fatal);
        object_property_set_int(obj, "mp-affinity", i << 8, &error_fatal);
        object_property_set_int(obj, "cntfrq", 12000000, &error_fatal);
        object_property_set_bool(obj, "start-powered-off", i != 0, &error_fatal);
        object_property_set_link(obj, "memory", OBJECT(sys), &error_fatal);
        /* A55 implements AArch32 debug ID; upstream model omitted it.
         * 6 breakpoints, 4 watchpoints, 2 context comparators, debug v8. */
        cpu->isar.dbgdidr = 0x3518d000;
        cpu->psci_conduit = QEMU_PSCI_CONDUIT_SMC;
        cpus[i] = DEVICE(obj);
        qdev_realize(cpus[i], NULL, &error_fatal);
    }
    DeviceState *gic = qdev_new("arm_gic");
    qdev_prop_set_uint32(gic, "revision", 2);
    qdev_prop_set_uint32(gic, "num-cpu", n);
    qdev_prop_set_uint32(gic, "num-irq", 256);
    qdev_prop_set_bit(gic, "has-security-extensions", false);
    sysbus_realize_and_unref(SYS_BUS_DEVICE(gic), &error_fatal);
    sysbus_mmio_map(SYS_BUS_DEVICE(gic), 0, 0x16001000);
    sysbus_mmio_map(SYS_BUS_DEVICE(gic), 1, 0x16002000);
    for (int i = 0; i < n; i++) {
        int base = 224 + i * 32;
        qdev_connect_gpio_out(cpus[i], GTIMER_PHYS, qdev_get_gpio_in(gic, base + 30));
        qdev_connect_gpio_out(cpus[i], GTIMER_VIRT, qdev_get_gpio_in(gic, base + 27));
        sysbus_connect_irq(SYS_BUS_DEVICE(gic), i, qdev_get_gpio_in(cpus[i], ARM_CPU_IRQ));
        sysbus_connect_irq(SYS_BUS_DEVICE(gic), i+n, qdev_get_gpio_in(cpus[i], ARM_CPU_FIQ));
    }
    piu_irq = qdev_get_gpio_in(gic, 64);
    piu_timer = timer_new_ns(QEMU_CLOCK_VIRTUAL, piu_expire, NULL);
    /* Vendor UART uses an 8-byte register stride within the RIU window. */
    serial_mm_init(sys, 0x1f201300, 3, qdev_get_gpio_in(gic, 38),
                   115200, serial_hd(0), DEVICE_LITTLE_ENDIAN);
}
static void g06_class_init(ObjectClass *oc, const void *data)
{
    MachineClass *mc = MACHINE_CLASS(oc);
    mc->desc = "Experimental G06 exact-kernel boot platform (partial RIU)";
    mc->init = g06_init;
    mc->max_cpus = 4;
    mc->default_cpus = 1;
    mc->default_cpu_type = ARM_CPU_TYPE_NAME("cortex-a55");
    mc->default_ram_size = 1430 * MiB;
}
static const TypeInfo g06_type = {
    .name = MACHINE_TYPE_NAME("g06"), .parent = TYPE_MACHINE,
    .class_init = g06_class_init, .instance_size = sizeof(MachineState),
    .interfaces = arm_machine_interfaces,
};
static void g06_register(void) { type_register_static(&g06_type); }
type_init(g06_register)
