/* SPDX-License-Identifier: MIT
 * QEMU-only probe using Android's stable C Binder ABI. Opaque declarations
 * correspond to frameworks/native/libs/binder/ndk/include_{ndk,platform}/android
 * in android-16.0.0_r2. No private C++ layouts or device interfaces are used.
 */
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

extern int puts(const char *);
extern int printf(const char *, ...);
extern int fflush(void *);
extern int strcmp(const char *, const char *);
extern int pause(void);
extern int usleep(unsigned);
extern int socketpair(int, int, int, int[2]);
extern int close(int);
extern int read(int, void *, unsigned);
extern int write(int, const void *, unsigned);
extern int memfd_create(const char *, unsigned);
extern int ftruncate(int, long);
extern void *mmap(void *, unsigned, int, int, int, long);
extern int munmap(void *, unsigned);
extern int pthread_create(long *, const void *, void *(*)(void *), void *);
extern int pthread_join(long, void **);

typedef struct AIBinder AIBinder;
typedef struct AIBinder_Class AIBinder_Class;
typedef struct AParcel AParcel;
extern AIBinder_Class *AIBinder_Class_define(const char *, void *(*)(void *),
    void (*)(void *), int32_t (*)(AIBinder *, uint32_t, const AParcel *, AParcel *));
extern AIBinder *AIBinder_new(const AIBinder_Class *, void *);
extern bool AIBinder_associateClass(AIBinder *, const AIBinder_Class *);
extern void AIBinder_decStrong(AIBinder *);
extern int32_t AServiceManager_addService(AIBinder *, const char *);
extern AIBinder *AServiceManager_checkService(const char *);
extern bool ABinderProcess_setThreadPoolMaxThreadCount(uint32_t);
extern void ABinderProcess_startThreadPool(void);
extern int32_t AIBinder_prepareTransaction(AIBinder *, AParcel **);
extern int32_t AIBinder_transact(AIBinder *, uint32_t, AParcel **, AParcel **, uint32_t);
extern int32_t AParcel_writeInt32(AParcel *, int32_t);
extern int32_t AParcel_readInt32(const AParcel *, int32_t *);
extern int32_t AParcel_writeParcelFileDescriptor(AParcel *, int);
extern int32_t AParcel_readParcelFileDescriptor(const AParcel *, int *);
extern void AParcel_delete(AParcel *);

#define CHECK(x) do { if (!(x)) { printf("NATIVE_PROBE_FAIL line=%d\n", __LINE__); fflush(NULL); return 1; } } while (0)

static void *create(void *arg) { return arg; }
static void destroy(void *arg) { (void)arg; }
static int32_t transact(AIBinder *binder, uint32_t code, const AParcel *in, AParcel *out)
{
    int32_t value;
    (void)binder;
    if (code == 2) {
        int fd;
        int32_t status = AParcel_readParcelFileDescriptor(in, &fd);
        if (status) return status;
        unsigned char *shared = mmap(NULL, 4096, 3 /* READ|WRITE */, 1 /* SHARED */, fd, 0);
        close(fd);
        if (shared == (void *)-1) return -12;
        for (unsigned i = 0; i < 4096; i++) shared[i] ^= 0x5a;
        munmap(shared, 4096);
        return AParcel_writeInt32(out, 4096);
    }
    if (code != 1) return -22;
    int32_t status = AParcel_readInt32(in, &value);
    return status ? status : AParcel_writeInt32(out, value + 1);
}

static void *thread(void *arg)
{
    *(int *)arg = 42;
    return arg;
}

struct worker { AIBinder *binder; int id; int ok; };
static void *binder_thread(void *arg)
{
    struct worker *worker = arg;
    worker->ok = 0;
    for (int i = 0; i < 128; ++i) {
        AParcel *in = NULL, *out = NULL;
        int32_t value = worker->id * 1000 + i, reply;
        if (AIBinder_prepareTransaction(worker->binder, &in) ||
            AParcel_writeInt32(in, value) ||
            AIBinder_transact(worker->binder, 1, &in, &out, 0)) return NULL;
        int32_t status = AParcel_readInt32(out, &reply);
        AParcel_delete(out);
        if (status || reply != value + 1) return NULL;
    }
    worker->ok = 1;
    return arg;
}

int main(int argc, char **argv)
{
    AIBinder_Class *cls = AIBinder_Class_define("org.dande.qemu.IProbe", create, destroy, transact);
    CHECK(cls);
    if (argc == 2 && !strcmp(argv[1], "server")) {
        AIBinder *binder = AIBinder_new(cls, NULL);
        CHECK(binder);
        CHECK(AServiceManager_addService(binder, "dande.qemu.probe") == 0);
        CHECK(ABinderProcess_setThreadPoolMaxThreadCount(1));
        ABinderProcess_startThreadPool();
        puts("ANDROID16_BINDER_SERVER_READY");
        fflush(NULL);
        for (;;) pause();
    }
    long tid;
    int value = 0;
    void *result = NULL;
    CHECK(pthread_create(&tid, NULL, thread, &value) == 0);
    CHECK(pthread_join(tid, &result) == 0 && result == &value && value == 42);
    int sockets[2];
    CHECK(socketpair(1 /* AF_UNIX */, 1 /* SOCK_STREAM */, 0, sockets) == 0);
    CHECK(write(sockets[0], "native", 6) == 6);
    char message[7] = {0};
    CHECK(read(sockets[1], message, 6) == 6 && !strcmp(message, "native"));
    close(sockets[0]);
    close(sockets[1]);
    puts("ANDROID16_THREADS_SOCKETS_OK");
    AIBinder *binder = NULL;
    for (int i = 0; i < 200 && !binder; ++i) {
        binder = AServiceManager_checkService("dande.qemu.probe");
        if (!binder) usleep(10000);
    }
    CHECK(binder);
    CHECK(AIBinder_associateClass(binder, cls));
    AParcel *in = NULL, *out = NULL;
    CHECK(AIBinder_prepareTransaction(binder, &in) == 0);
    CHECK(AParcel_writeInt32(in, 41) == 0);
    CHECK(AIBinder_transact(binder, 1, &in, &out, 0) == 0);
    CHECK(AParcel_readInt32(out, &value) == 0 && value == 42);
    AParcel_delete(out);
    puts("ANDROID16_BINDER_ROUNDTRIP=42");

    int fd = memfd_create("dande-binder-probe", 1 /* CLOEXEC */);
    CHECK(fd >= 0 && ftruncate(fd, 4096) == 0);
    unsigned char *shared = mmap(NULL, 4096, 3, 1, fd, 0);
    CHECK(shared != (void *)-1);
    for (unsigned i = 0; i < 4096; ++i) shared[i] = (unsigned char)i;
    in = out = NULL;
    CHECK(AIBinder_prepareTransaction(binder, &in) == 0);
    CHECK(AParcel_writeParcelFileDescriptor(in, fd) == 0);
    CHECK(AIBinder_transact(binder, 2, &in, &out, 0) == 0);
    CHECK(AParcel_readInt32(out, &value) == 0 && value == 4096);
    for (unsigned i = 0; i < 4096; ++i) CHECK(shared[i] == ((unsigned char)i ^ 0x5a));
    AParcel_delete(out);
    CHECK(munmap(shared, 4096) == 0 && close(fd) == 0);
    puts("ANDROID16_BINDER_SHARED_MEMORY_OK");

    long tids[4];
    struct worker workers[4];
    for (int i = 0; i < 4; ++i) {
        workers[i] = (struct worker){binder, i, 0};
        CHECK(pthread_create(&tids[i], NULL, binder_thread, &workers[i]) == 0);
    }
    for (int i = 0; i < 4; ++i) {
        CHECK(pthread_join(tids[i], &result) == 0 && result == &workers[i] && workers[i].ok);
    }
    AIBinder_decStrong(binder);
    puts("ANDROID16_BINDER_CONCURRENT_512_OK");
    return 0;
}
