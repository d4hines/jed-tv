// SPDX-License-Identifier: GPL-2.0-only
/* QEMU-only regression probe for the timer shutdown prerequisite. */
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/jiffies.h>
#include <linux/module.h>
#include <linux/timer.h>

static struct timer_list probe;

static void callback(struct timer_list *timer)
{
}

#define CHECK(condition) do { if (!(condition)) goto fail; } while (0)

static int __init probe_init(void)
{
	timer_setup(&probe, callback, 0);
	CHECK(mod_timer(&probe, jiffies + 60 * HZ) == 0);
	CHECK(timer_shutdown_sync(&probe) == 1);
	CHECK(!timer_pending(&probe) && !probe.function);
	CHECK(mod_timer(&probe, jiffies + 60 * HZ) == 0);
	CHECK(!timer_pending(&probe));
	probe.expires = jiffies + 60 * HZ;
	add_timer_on(&probe, 0);
	CHECK(!timer_pending(&probe));

	timer_setup(&probe, callback, 0);
	CHECK(mod_timer(&probe, jiffies + 60 * HZ) == 0);
	CHECK(timer_shutdown(&probe) == 1);
	CHECK(!timer_pending(&probe) && !probe.function);
	CHECK(mod_timer(&probe, jiffies + 60 * HZ) == 0);
	CHECK(!timer_pending(&probe));

	/* Existing delete APIs allow a subsequent rearm. */
	timer_setup(&probe, callback, 0);
	CHECK(mod_timer(&probe, jiffies + 60 * HZ) == 0);
	CHECK(del_timer_sync(&probe) == 1);
	CHECK(probe.function == callback);
	CHECK(mod_timer(&probe, jiffies + 60 * HZ) == 0);
	CHECK(del_timer(&probe) == 1);
	timer_shutdown_sync(&probe);
	pr_info("KERNEL_TIMER_SHUTDOWN_OK\n");
	return 0;
fail:
	timer_shutdown_sync(&probe);
	pr_err("KERNEL_TIMER_SHUTDOWN_FAIL\n");
	return -EINVAL;
}

static void __exit probe_exit(void)
{
	timer_shutdown_sync(&probe);
}

module_init(probe_init);
module_exit(probe_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("QEMU-only timer shutdown regression probe");
