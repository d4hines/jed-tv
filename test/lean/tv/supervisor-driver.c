/* Test transport. Decision policy is the generated Lean code. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
extern uint64_t tv_supervisor_step(uint32_t,uint64_t,uint64_t,uint32_t,uint32_t);
extern uint64_t tv_supervisor_deadline(uint32_t,uint64_t,uint64_t,uint32_t,uint32_t);
extern uint32_t tv_supervisor_fork(uint32_t);
extern uint32_t tv_supervisor_wait(uint32_t,uint32_t,uint32_t);
extern uint64_t tv_supervisor_status(uint32_t);
int main(void) {
    uint32_t op; uint64_t a,b,c,d,e;
    while (scanf("%"SCNu32" %"SCNu64" %"SCNu64" %"SCNu64" %"SCNu64" %"SCNu64,&op,&a,&b,&c,&d,&e)==6) {
        uint64_t value=0,deadline=0;
        switch(op) {
        case 0: value=tv_supervisor_step(a,b,c,d,e); deadline=tv_supervisor_deadline(a,b,c,d,e); break;
        case 1: value=tv_supervisor_fork(a); break;
        case 2: value=tv_supervisor_wait(a,b,c); break;
        case 3: value=tv_supervisor_status(a); break;
        default: return 2;
        }
        printf("%"PRIu64" %"PRIu64"\n",value,deadline);
    }
    return 0;
}
