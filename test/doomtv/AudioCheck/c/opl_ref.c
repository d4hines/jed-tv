/*
 * Host reference harness for Doom.Audio.Opl: drives a Nuked OPL3 C
 * implementation (upstream or the Chocolate Doom fork) with a register
 * trace and writes the output as raw s16le stereo.
 *
 * usage: opl_ref native|stream <rate> <frames> <trace.txt> <out.raw>
 *   trace lines: "<frame> <reg> <value>" (decimal), sorted by frame.
 *   native: OPL3_WriteReg before OPL3_Generate of that 49716 Hz sample.
 *   stream: OPL3_WriteRegBuffered before OPL3_GenerateResampled of that frame
 *           (exactly how Chocolate Doom's opl_sdl.c drives the chip).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "opl3.h"

static opl3_chip chip;

int main(int argc, char **argv)
{
    if (argc != 6) {
        fprintf(stderr, "usage: %s native|stream rate frames trace out\n", argv[0]);
        return 2;
    }
    int stream = strcmp(argv[1], "stream") == 0;
    unsigned rate = (unsigned) atoi(argv[2]);
    long frames = atol(argv[3]);
    FILE *tf = fopen(argv[4], "r");
    FILE *of = fopen(argv[5], "wb");
    if (!tf || !of) { perror("open"); return 1; }
    OPL3_Reset(&chip, rate);
    long nf; unsigned reg, val;
    int have = fscanf(tf, "%ld %u %u", &nf, &reg, &val) == 3;
    int16_t buf[2];
    for (long f = 0; f < frames; f++) {
        while (have && nf == f) {
            if (stream) OPL3_WriteRegBuffered(&chip, (uint16_t) reg, (uint8_t) val);
            else OPL3_WriteReg(&chip, (uint16_t) reg, (uint8_t) val);
            have = fscanf(tf, "%ld %u %u", &nf, &reg, &val) == 3;
        }
        if (stream) OPL3_GenerateResampled(&chip, buf);
        else OPL3_Generate(&chip, buf);
        fwrite(buf, sizeof(int16_t), 2, of);
    }
    fclose(of);
    fclose(tf);
    return 0;
}
