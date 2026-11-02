#include <stdint.h>
uint8_t leanboot_model_read(uint32_t b, uint32_t s, uint32_t p) {
    (void)b; (void)s; (void)p; __builtin_trap();
}
uint8_t leanboot_model_write(uint32_t b, uint32_t s, uint32_t p) {
    (void)b; (void)s; (void)p; __builtin_trap();
}
uint8_t leanboot_model_flush(void) { __builtin_trap(); }
