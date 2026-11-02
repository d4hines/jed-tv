/* Authored PID1 native mechanisms live here. Permission/filter policy is Lean.
 * No Lean allocation/runtime is entered between fork and exec. */
#define _GNU_SOURCE
#include <errno.h>
#include <dirent.h>
#include <fcntl.h>
#include <grp.h>
#include <linux/filter.h>
#include <linux/seccomp.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/prctl.h>
#include <sys/mount.h>
#include <sched.h>
#include <sys/resource.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

extern uint32_t tv_app_uid(uint32_t);
extern uint32_t tv_app_threads(uint32_t);
extern uint32_t tv_app_gid(uint32_t);
extern uint64_t tv_app_limit(uint32_t, uint32_t);
extern uint32_t tv_sandbox_length(uint32_t, uint32_t, uint32_t);
extern uint64_t tv_sandbox_instruction(uint32_t, uint32_t, uint32_t, uint32_t);

/* After fork only raw syscalls and local arithmetic are permitted. In
 * particular, stdio and the Lean/runtime allocator may hold another thread's
 * locks at fork time. Keep even the failure path independent of them. */
static void child_write(const char *bytes, size_t length) {
    while (length) {
        long n = syscall(SYS_write, 2, bytes, length);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) return;
        bytes += n;
        length -= (size_t)n;
    }
}
__attribute__((noreturn))
static void child_fail(const char *stage, size_t length, int error) {
    static const char prefix[] = "tvinit: app launch failed at ";
    static const char suffix[] = ": errno ";
    char digits[10];
    size_t end = sizeof(digits), begin = end;
    unsigned number = (unsigned)error;
    do { digits[--begin] = (char)('0' + number % 10); number /= 10; } while (number);
    child_write(prefix, sizeof(prefix) - 1);
    child_write(stage, length);
    child_write(suffix, sizeof(suffix) - 1);
    child_write(digits + begin, end - begin);
    child_write("\n", 1);
    syscall(SYS_exit_group, 126);
    __builtin_unreachable();
}
#define fail(stage) child_fail(stage, sizeof(stage) - 1, errno)

struct launch_policy {
    uint32_t uid, gid;
    uint64_t limits[4], close_bound;
    char root[64], devices[80], input[96], commandline[96];
    struct sock_filter instructions[256];
    struct sock_fprog program;
};

static int close_bound(uint64_t *bound) {
    struct rlimit files;
    if (getrlimit(RLIMIT_NOFILE, &files)) return -errno;
    /* Linux descriptors are signed ints. Include every descriptor another
     * thread could allocate before fork, not merely the current fd snapshot. */
    *bound = files.rlim_max == RLIM_INFINITY || (uint64_t)files.rlim_max > INT_MAX
        ? (uint64_t)INT_MAX + 1 : (uint64_t)files.rlim_max;
    DIR *dir = opendir("/proc/self/fd");
    if (!dir) return -errno;
    struct dirent *entry;
    for (;;) {
        errno = 0;
        entry = readdir(dir);
        if (!entry) {
            int error = errno;
            closedir(dir);
            if (error) return -error;
            break;
        }
        char *end;
        long fd = strtol(entry->d_name, &end, 10);
        if (!*end && fd >= 0 && fd <= INT_MAX && (uint64_t)fd >= *bound)
            *bound = (uint64_t)fd + 1;
    }
    if (*bound < 1024) *bound = 1024;
    return 0;
}

static void child_close_fds(uint64_t bound) {
#ifdef SYS_close_range
    /* Newer QEMU kernels can do this cheaply. The TV's4.19 kernel falls back
     * to close(), with the upper bound computed by the parent. */
    int low = syscall(SYS_close_range, 5u, 1022u, 0u) == 0;
    int high = syscall(SYS_close_range, 1024u, UINT_MAX, 0u) == 0;
    if (low && high) return;
#endif
    for (uint64_t fd = 5; fd < bound; ++fd)
        if (fd != 1023) syscall(SYS_close, (int)fd);
}

static void child_limit(int resource, uint64_t n) {
    /* prlimit64 has the same two-u64 wire layout on ARM32 and AArch64. */
    struct { uint64_t current, maximum; } r = { n, n };
    if (syscall(SYS_prlimit64, 0, resource, &r, NULL)) fail("setrlimit");
}

/* The descriptor gate also needs a filesystem boundary: execveat ignores
 * dirfd for absolute paths. Bind a nonrecursive noexec root; never expose a
 * live procfs (notably /proc/self/exe) or any executable mount to the app. */
static void private_view(const struct launch_policy *p) {
    if (syscall(SYS_mkdirat, AT_FDCWD, "/run", 0755) && errno != EEXIST) fail("run directory");
    if (syscall(SYS_mkdirat, AT_FDCWD, p->root, 0700) && errno != EEXIST) fail("app root");
    if (syscall(SYS_unshare, CLONE_NEWNS)) fail("mount namespace");
    if (syscall(SYS_mount, NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL)) fail("private mounts");
    if (syscall(SYS_mount, "/", p->root, NULL, MS_BIND, NULL)) fail("root bind");
    if (syscall(SYS_mount, NULL, p->root, NULL, MS_BIND | MS_REMOUNT | MS_NOEXEC | MS_NOSUID | MS_NODEV, NULL)) fail("noexec root");
    if (syscall(SYS_mount, "/dev", p->devices, NULL, MS_BIND, NULL)) fail("device bind");
    if (syscall(SYS_mount, NULL, p->devices, NULL, MS_BIND | MS_REMOUNT | MS_NOEXEC | MS_NOSUID, NULL)) fail("noexec devices");
    if (syscall(SYS_mount, "tmpfs", p->input, "tmpfs", MS_NOEXEC | MS_NOSUID | MS_NODEV, "size=4k,mode=000")) fail("hide raw input");
    /* Preserve configuration as inert text, without exposing live procfs. */
    int source = (int)syscall(SYS_openat, AT_FDCWD, "/proc/cmdline", O_RDONLY | O_CLOEXEC, 0);
    if (source < 0) fail("read command line");
    int target = (int)syscall(SYS_openat, AT_FDCWD, p->commandline, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0444);
    if (target < 0) fail("command line snapshot");
    char bytes[4096]; ssize_t n;
    while ((n = syscall(SYS_read, source, bytes, sizeof(bytes))) > 0) {
        ssize_t offset = 0;
        while (offset < n) {
            ssize_t written = syscall(SYS_write, target, bytes + offset, (size_t)(n - offset));
            if (written < 0 && errno == EINTR) continue;
            if (written <= 0) fail("command line copy");
            offset += written;
        }
    }
    if (n < 0) fail("command line read");
    syscall(SYS_close, source); syscall(SYS_close, target);
    if (syscall(SYS_chroot, p->root)) fail("chroot");
}

/* Returns pid, or negative errno. path/argv/env are launcher-owned immutable
 * storage. The remote input socket becomes fd 3 and the app's PID 1 request
 * socket fd 4; no other launcher fds survive. */
int32_t tv_linux_launch(const char *path, char *const argv[], char *const env[],
                        uint32_t app, uint32_t grants, int input, int requests, const char *workdir) {
    struct launch_policy p;
    p.uid = tv_app_uid(app);
    p.gid = tv_app_gid(app);
    if (!p.uid || !tv_app_threads(app)) return -EINVAL;
    for (uint32_t i = 0; i < 4; ++i) p.limits[i] = tv_app_limit(app, i);
    if (p.limits[0] >= 1023) return -EINVAL;
#if defined(__arm__)
    uint32_t arch = 0;
#elif defined(__aarch64__)
    uint32_t arch = 1;
#else
#error Unsupported PID1 sandbox architecture
#endif
    uint32_t count = tv_sandbox_length(app, grants, arch);
    if (!count || count > 256) return -EINVAL;
    for (uint32_t i = 0; i < count; ++i) {
        uint64_t word = tv_sandbox_instruction(app, grants, arch, i);
        p.instructions[i] = (struct sock_filter){word, word >> 16, word >> 24, word >> 32};
    }
    p.program = (struct sock_fprog){count, p.instructions};
#define format_path(field, ...) do { \
    int length = snprintf(p.field, sizeof(p.field), __VA_ARGS__); \
    if (length < 0 || (size_t)length >= sizeof(p.field)) return -ENAMETOOLONG; \
} while (0)
    format_path(root, "/run/tv-app-%u", p.uid);
    format_path(devices, "%s/dev", p.root);
    format_path(input, "%s/input", p.devices);
    format_path(commandline, "%s/proc/cmdline", p.root);
#undef format_path
    int result = close_bound(&p.close_bound);
    if (result) return result;
    int executable = open(path, O_RDONLY | O_CLOEXEC);
    if (executable < 0) return -errno;
    int reserved = fcntl(executable, F_DUPFD_CLOEXEC, 1023);
    int reserve_error = errno;
    close(executable);
    if (reserved < 0) return -reserve_error;
    if (reserved != 1023) { close(reserved); return -EMFILE; }
    pid_t pid = fork();
    if (pid < 0) { int e = errno; close(reserved); return -e; }
    if (pid) { close(reserved); return pid; }

    if (requests == 3 && (requests = (int)syscall(SYS_fcntl, 3, F_DUPFD, 5)) < 0) fail("request fd");
    if (input != 3 && syscall(SYS_dup3, input, 3, 0) < 0) fail("input fd");
    if (requests != 4 && syscall(SYS_dup3, requests, 4, 0) < 0) fail("request fd");
    if (syscall(SYS_fcntl, 3, F_SETFD, 0) || syscall(SYS_fcntl, 4, F_SETFD, 0)) fail("descriptor CLOEXEC");
    child_close_fds(p.close_bound);
    private_view(&p);
    if (syscall(SYS_chdir, workdir)) fail("workdir");
    child_limit(RLIMIT_CORE, 0);
    child_limit(RLIMIT_NOFILE, p.limits[0]);
    child_limit(RLIMIT_NPROC, p.limits[1]);
    child_limit(RLIMIT_AS, p.limits[2]);
    child_limit(RLIMIT_FSIZE, p.limits[3]);
    for (int capability = 0; capability < 64; ++capability)
        if (syscall(SYS_prctl, PR_CAPBSET_DROP, capability, 0, 0, 0) && errno != EINVAL) fail("capability drop");
#if defined(__arm__)
#define setresgid_call SYS_setresgid32
#define setresuid_call SYS_setresuid32
#else
#define setresgid_call SYS_setresgid
#define setresuid_call SYS_setresuid
#endif
    if (syscall(SYS_setgroups, 0, NULL)
        || syscall(setresgid_call, p.gid, p.gid, p.gid)
        || syscall(setresuid_call, p.uid, p.uid, p.uid)) fail("identity");
    if (syscall(SYS_prctl, PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) fail("no_new_privs");
    if (syscall(SYS_prctl, PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &p.program)) fail("seccomp");
    syscall(SYS_execveat, reserved, "", argv, env, AT_EMPTY_PATH);
    fail("execveat");
}
