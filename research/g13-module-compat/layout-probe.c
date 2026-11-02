/* Compile-only constants: no module is loaded and no hardware is accessed. */
#include <linux/module.h>
#include <linux/sched.h>
#include <linux/mm_types.h>
#include <linux/stddef.h>

const unsigned int g13_sizeof_module = sizeof(struct module);
const unsigned int g13_module_init = offsetof(struct module, init);
const unsigned int g13_module_exit = offsetof(struct module, exit);
const unsigned int g13_sizeof_task = sizeof(struct task_struct);
const unsigned int g13_task_usage = offsetof(struct task_struct, usage);
const unsigned int g13_task_mm = offsetof(struct task_struct, mm);
const unsigned int g13_task_comm = offsetof(struct task_struct, comm);
const unsigned int g13_sizeof_mm = sizeof(struct mm_struct);
const unsigned int g13_mm_cpu_bitmap = offsetof(struct mm_struct, cpu_bitmap);
