/* Thin memory boundary around pinned HACL* Ed25519/SHA-256. No signing key,
 * allocation, I/O, parser, policy, or storage operation belongs here. */
#include <stdint.h>
#include <stddef.h>
#include "Hacl_Ed25519.h"
#include "Hacl_Hash_SHA2.h"
#include "update-key.h"

uint8_t leanboot_update_signature(uint32_t header) {
    uint8_t *p = (uint8_t *)(uintptr_t)header;
    return Hacl_Ed25519_verify((uint8_t *)leanboot_update_key, 96, p, p + 96);
}
uint8_t leanboot_update_digest(uint32_t input, uint32_t length, uint32_t expected) {
    uint8_t digest[32];
    Hacl_Hash_SHA2_hash_256(digest, (uint8_t *)(uintptr_t)input, length);
    uint8_t different = 0;
    for (unsigned i = 0; i < 32; ++i)
        different |= digest[i] ^ ((const uint8_t *)(uintptr_t)expected)[i];
    return different == 0;
}

/* Compiler-generated memory operations in HACL*, implemented without libc.
 * All pointers/counts remain within the caller-owned crypto buffers/stack. */
void *memcpy(void *dst, const void *src, size_t n) {
    uint8_t *d = dst; const uint8_t *s = src;
    for (size_t i = 0; i < n; ++i) d[i] = s[i];
    return dst;
}
void *memset(void *dst, int c, size_t n) {
    uint8_t *d = dst;
    for (size_t i = 0; i < n; ++i) d[i] = (uint8_t)c;
    return dst;
}
void __aeabi_memcpy(void *d, const void *s, size_t n) { memcpy(d, s, n); }
void __aeabi_memcpy4(void *d, const void *s, size_t n) { memcpy(d, s, n); }
void __aeabi_memclr(void *d, size_t n) { memset(d, 0, n); }
void __aeabi_memclr4(void *d, size_t n) { memset(d, 0, n); }
void __aeabi_memclr8(void *d, size_t n) { memset(d, 0, n); }
