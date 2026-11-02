/* Host (little-endian) stand-in for the SDL byte-swap macros used by
   Chocolate Doom's i_swap.h and midifile.c. */
#ifndef AUDIOCHECK_SDL_ENDIAN_H
#define AUDIOCHECK_SDL_ENDIAN_H
#include <stdint.h>
#define SDL_SwapLE16(x) ((uint16_t)(x))
#define SDL_SwapLE32(x) ((uint32_t)(x))
#define SDL_SwapBE16(x) ((uint16_t)((((uint16_t)(x)) >> 8) | (((uint16_t)(x)) << 8)))
#define SDL_SwapBE32(x) __builtin_bswap32((uint32_t)(x))
#endif
