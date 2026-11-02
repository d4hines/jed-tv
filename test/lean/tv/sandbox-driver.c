#include <stdint.h>
#include <stdio.h>
extern uint32_t tv_app_uid(uint32_t);
extern uint32_t tv_app_threads(uint32_t);
extern uint64_t tv_app_limit(uint32_t, uint32_t);
extern uint32_t tv_sandbox_length(uint32_t, uint32_t, uint32_t);
extern uint64_t tv_sandbox_instruction(uint32_t, uint32_t, uint32_t, uint32_t);
int main(void) {
    for (unsigned app=0; app<9; ++app) for (unsigned arch=0; arch<3; ++arch) {
        /* Every grant on (1 internet, 2 local network, 4 saved data). */
        unsigned n=tv_sandbox_length(app,7,arch);
        printf("%u %u %u %u %llu %u", app,arch,tv_app_uid(app),tv_app_threads(app),
            (unsigned long long)tv_app_limit(app,0),n);
        for (unsigned i=0; i<n; ++i) printf(" %llu",(unsigned long long)tv_sandbox_instruction(app,7,arch,i));
        putchar('\n');
    }
}
