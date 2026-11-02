/* SPDX-License-Identifier: MIT
 * QEMU-only regression probe for the stable DONTUNMAP/VMA merge fixes.
 * Uses the Android Bionic runtime already present in the small test ramdisk.
 */
#include <stddef.h>
extern void *mmap(void *, size_t, int, int, int, long);
extern void *mremap(void *, size_t, size_t, int, ...);
extern int munmap(void *, size_t);
extern int printf(const char *, ...);

#define PAGE 4096
#define CHECK(x) do { if (!(x)) { printf("KERNEL_PROBE_FAIL line=%d\n", __LINE__); return 1; } } while (0)

int main(void)
{
    unsigned cases = 0;
    for (unsigned right = 0; right < 2; ++right)
    for (unsigned faulted = 0; faulted < 2; ++faulted)
    for (unsigned partial = 0; partial < 2; ++partial) {
        unsigned char *area = mmap(NULL, 6 * PAGE, 0, 0x22, -1, 0);
        CHECK(area != (void *)-1);
        unsigned char *source = area + 2 * PAGE;
        unsigned length = (1 + partial) * PAGE;
        CHECK(mmap(source, length, 3, 0x32, -1, 0) == source);
        unsigned char *old = source + (right && partial ? PAGE : 0);
        unsigned char *destination = right ? source + length : source - PAGE;
        if (faulted)
            for (unsigned i = 0; i < PAGE; ++i) old[i] = (unsigned char)(i ^ 0x5a);
        /* MAYMOVE | FIXED | DONTUNMAP; destination abuts the original VMA. */
        CHECK(mremap(old, PAGE, PAGE, 7, destination) == destination);
        for (unsigned i = 0; i < PAGE; ++i) {
            CHECK(destination[i] == (faulted ? (unsigned char)(i ^ 0x5a) : 0));
            CHECK(old[i] == 0);
        }
        old[0] = 0x31;
        CHECK(destination[0] == (faulted ? 0x5a : 0));
        destination[0] = 0x42;
        CHECK(old[0] == 0x31);
        CHECK(munmap(area, 6 * PAGE) == 0);
        ++cases;
    }
    printf("MREMAP_DONTUNMAP_CASES=%u\n", cases);
    return 0;
}
