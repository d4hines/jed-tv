extern void *dlopen(const char *, int);
extern void *dlsym(void *, const char *);
extern char *dlerror(void);
extern int printf(const char *, ...);
extern int fflush(void *);
extern int strcmp(const char *, const char *);
int main(int argc, char **argv) {
    void *lib=dlopen("/stock-vendor/tvos/bin/sitatvservice",2);
    if(!lib){ printf("dlopen: %s\n",dlerror()); return 1; }
    printf("TCL middleware loaded; amp=%p config=%p\n",dlsym(lib,"fpi_audio_amplifier_init"),dlsym(lib,"config_init"));fflush(0);
    if(argc>1 && (!strcmp(argv[1],"--config") || !strcmp(argv[1],"--amp"))) {
       int (*init)(void)=dlsym(lib,"config_init");
       if(!init) return 2;
       printf("config_init=%d\n",init());fflush(0);
       char *(*get)(const char *,int *,const char *)=dlsym(lib,"config_get_str");
       int length=0;
       char *value=get("model.Drives.AMP_CHIPS.AMP_1.TYPE",&length,0);
       printf("model.Drives.AMP_CHIPS.AMP_1.TYPE length=%d value=%s\n",length,value?value:"(null)");fflush(0);
       if(!strcmp(argv[1],"--amp")) {
           if(!value || strcmp(value,"AD82111")) return 3;
           int (*gpio)(void)=dlsym(lib,"fpi_gpio_init");
           int (*amp)(int)=dlsym(lib,"fpi_audio_amplifier_init");
           if(!gpio || !amp) return 4;
           void *fpp=dlopen("/vendor/tvos/libBionic/libfpp.so",2);
           const char *stages[]={"mif_handle_init","mif_system_init","mif_gpio_init","mif_iic_init"};
           for(unsigned i=0;i<sizeof(stages)/sizeof(stages[0]);i++) {
               int (*stage)(void)=fpp?dlsym(fpp,stages[i]):0;
               if(!stage) return 5;
               int result=stage();
               printf("%s=%d\n",stages[i],result);fflush(0);
               if(result!=0 && result!=-2) return 6;
           }
           printf("fpi_gpio_init=%d\n",gpio());fflush(0);
           int (*reset)(void)=dlsym(lib,"fpi_amp_reset_pd");
           if(!reset || reset()!=0) return 8;
           int result=amp(0);
           printf("fpi_audio_amplifier_init=%d\n",result);fflush(0);
           if(result!=0) return 9;
           int (*unmute)(int)=dlsym(lib,"fpi_amp_mute");
           if(!unmute) return 7;
           printf("fpi_amp_mute(0)=%d\n",unmute(0));fflush(0);
       }
    }
    return 0;
}
