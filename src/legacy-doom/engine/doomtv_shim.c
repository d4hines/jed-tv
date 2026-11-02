/* doomtv_shim.c: the only C we add to the engine. It keeps the Doom engine's
 * exit() calls from ever ending the process: when doomtv runs as PID 1, an
 * exiting init panics the kernel.
 *
 * app/doom/build.sh compiles every engine file with -Dexit=doomtv_exit and
 * -Dsystem=doomtv_system. doomtv_exit() longjmps back to the setjmp in
 * doomtv_create()/doomtv_tick(), which the Rust main loop calls. Only C
 * frames lie between the setjmp and the longjmp: the Rust DG_* hooks never
 * call back into the engine, so no Rust frame is ever skipped. */
#include <setjmp.h>
#include <stdio.h>
#include "doomgeneric.h"
#include "i_system.h"

static jmp_buf *guard;
static int exit_code;

_Noreturn void doomtv_exit(int code)
{
    fflush(stdout);
    fflush(stderr);
    if (guard) {
        exit_code = code;
        longjmp(*guard, 1);
    }
    /* Unreachable: the engine only runs inside doomtv_create/doomtv_tick. */
    fprintf(stderr, "doomtv: exit(%d) outside the engine guard; halting the engine\n", code);
    for (;;) {
    }
}

/* I_Error tries to pop up a zenity dialog with system(); there is no shell. */
int doomtv_system(const char *cmd)
{
    (void)cmd;
    return -1;
}

/* I_Quit runs the I_AtExit list (newest first) and, in doomgeneric, relies on
 * D_Endoom's exit(0) to finish. Registered before the engine registers
 * anything, this runs last and makes every I_Quit end the engine even when
 * the ENDOOM screen is disabled. */
static void quit_last(void)
{
    doomtv_exit(0);
}

/* Each returns 0 while the engine keeps running, or 1 after the engine called
 * exit(); *code is then its exit status (0 = quit from the menu). */
int doomtv_create(int argc, char **argv, int *code)
{
    jmp_buf jb;
    if (setjmp(jb)) {
        guard = NULL;
        *code = exit_code;
        return 1;
    }
    guard = &jb;
    I_AtExit(quit_last, false);
    doomgeneric_Create(argc, argv);
    guard = NULL;
    return 0;
}

int doomtv_tick(int *code)
{
    jmp_buf jb;
    if (setjmp(jb)) {
        guard = NULL;
        *code = exit_code;
        return 1;
    }
    guard = &jb;
    doomgeneric_Tick();
    guard = NULL;
    return 0;
}
