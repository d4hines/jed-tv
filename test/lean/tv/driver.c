/* Differential-test transport only; all menu decisions come from Lean C. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
extern uint64_t tv_menu_step(uint64_t, uint64_t, uint32_t, uint32_t);
int main(void) {
    uint64_t state, shape; uint32_t key, repeated;
    while (scanf("%" SCNu64 " %" SCNu64 " %" SCNu32 " %" SCNu32, &state, &shape, &key, &repeated)==4)
        printf("%" PRIu64 "\n",tv_menu_step(state,shape,key,repeated));
    return 0;
}
