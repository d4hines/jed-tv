/* Offline exact-kernel ABI smoke test, then replace PID 1 with real tvinit. */
#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/random.h>
#include <sys/utsname.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
static void *worker(void *arg) { return arg; }
static void require(int ok, const char *what) {
    if (!ok) { printf("G06-SMOKE FAIL %s errno=%d\n", what, errno); fflush(stdout); for (;;) pause(); }
}
int main(void) {
    setvbuf(stdout, NULL, _IONBF, 0);
    struct utsname u; require(uname(&u)==0, "uname");
    printf("G06-SMOKE pid=%d kernel=%s machine=%s cpus=%ld\n", getpid(), u.release, u.machine, sysconf(_SC_NPROCESSORS_ONLN));
    require(getpid()==1 && strcmp(u.release,"4.19.116+")==0, "stock kernel PID1");
    pthread_t t; void *v;
    require(pthread_create(&t,0,worker,(void *)42)==0, "pthread_create");
    require(pthread_join(t,&v)==0 && v==(void *)42, "pthread_join");
    struct timespec a,b,delay={0,100000000};
    require(clock_gettime(CLOCK_MONOTONIC,&a)==0, "clock");
    while (nanosleep(&delay,&delay)<0) require(errno==EINTR,"nanosleep");
    require(clock_gettime(CLOCK_MONOTONIC,&b)==0 && (b.tv_sec>a.tv_sec || b.tv_nsec>a.tv_nsec),"time advances");
    pid_t p=fork(); require(p>=0,"fork"); if (!p) _exit(23);
    int status; require(waitpid(p,&status,0)==p && WIFEXITED(status) && WEXITSTATUS(status)==23,"waitpid");
    unsigned char random[16]; int n=getrandom(random,sizeof(random),GRND_NONBLOCK);
    printf("G06-SMOKE randomness=%s\n",n==sizeof(random)?"ready":"not-ready");
    puts("G06-SMOKE ABI PASS; exec real tvinit as PID1");
    execl("/bin/tvinit","tvinit",(char *)0); require(0,"exec tvinit");
}
