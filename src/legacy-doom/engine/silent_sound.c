#include "s_sound.h"

int snd_channels = 0;
int sfxVolume = 0;
int musicVolume = 0;

void S_Init(int sfx, int music) { (void)sfx; (void)music; }
void S_Shutdown(void) {}
void S_Start(void) {}
void S_StartSound(void *origin, int id) { (void)origin; (void)id; }
void S_StopSound(mobj_t *origin) { (void)origin; }
void S_StartMusic(int id) { (void)id; }
void S_ChangeMusic(int id, int looping) { (void)id; (void)looping; }
boolean S_MusicPlaying(void) { return false; }
void S_StopMusic(void) {}
void S_PauseSound(void) {}
void S_ResumeSound(void) {}
void S_UpdateSounds(mobj_t *listener) { (void)listener; }
void S_SetMusicVolume(int volume) { (void)volume; }
void S_SetSfxVolume(int volume) { (void)volume; }
