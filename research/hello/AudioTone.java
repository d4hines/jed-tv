/** One second maximum, with quiet PCM and a bounded playback lifetime. */
public final class AudioTone {
    public static void main(String[] args) throws Exception {
        Thread.setDefaultUncaughtExceptionHandler((thread, error) -> {
            error.printStackTrace(); Runtime.getRuntime().halt(1);
        });
        prepareAudio();
        Class<?> type = Class.forName("android.media.AudioTrack");
        int percent = args.length == 0 ? 3 : Integer.parseInt(args[0]);
        if (percent < 1 || percent > 30) throw new IllegalArgumentException("Volume must be 1..30 percent");
        int rate = 48000;
        boolean jingle = args.length > 1 && args[1].equals("jingle");
        int minimum = (Integer) type.getMethod("getMinBufferSize", int.class, int.class, int.class).invoke(null, rate, 12, 2);
        if (minimum < 0) throw new IllegalStateException("Audio buffer error " + minimum);
        Object track = type.getConstructor(int.class,int.class,int.class,int.class,int.class,int.class)
            .newInstance(3,rate,12,2,Math.max(minimum,rate*4),0);
        try {
            int state = (Integer)type.getMethod("getState").invoke(track);
            if (state != 1 && state != 2) throw new IllegalStateException("AudioTrack not initialized: " + state);
            short[] samples = new short[rate * 2];
            double[] melody = {523.25, 659.25, 783.99, 1046.50};
            for (int n=0; n<rate; n++) {
                int noteLength = rate / melody.length;
                int local = jingle ? n % noteLength : n;
                int length = jingle ? noteLength : rate;
                double edge = Math.min(1.0, Math.min(local,length-1-local)/480.0);
                double frequency = jingle ? melody[n / noteLength] : 440;
                double phase = 2*Math.PI*frequency*local/rate;
                double wave = jingle ? (Math.sin(phase) + 0.25*Math.sin(2*phase))/1.25 : Math.sin(phase);
                short value = (short)(wave*32767*(percent/100.0)*edge);
                samples[n*2]=value; samples[n*2+1]=value;
            }
            type.getMethod("setVolume",float.class).invoke(track,1.0f);
            int wrote=(Integer)type.getMethod("write",short[].class,int.class,int.class).invoke(track,samples,0,samples.length);
            if(wrote!=samples.length) throw new IllegalStateException("Short audio write: "+wrote);
            for (java.lang.reflect.Field f : Class.forName("android.media.PlayerBase").getDeclaredFields()) {
                if (f.getName().contains("AppOps") || f.getName().contains("Restricted")) {
                    f.setAccessible(true); System.out.println(f.getName()+"="+f.get(track));
                }
            }
            System.out.println("TONE playing: 1 second, "+percent+"% PCM amplitude, "+(jingle ? "four-note rising jingle" : "440 Hz")); System.out.flush();
            type.getMethod("play").invoke(track);
            Thread.sleep(1000);
            System.out.println("TONE playback frames="+type.getMethod("getPlaybackHeadPosition").invoke(track));
            type.getMethod("stop").invoke(track);
            System.out.println("TONE stopped");
        } finally { type.getMethod("release").invoke(track); }
    }
    public static void prepareAudio() throws Exception {
        // Only application/player bookkeeping; audio still uses real AudioFlinger.
        Class<?> service = Class.forName("android.media.IAudioService");
        Object bookkeeping = java.lang.reflect.Proxy.newProxyInstance(service.getClassLoader(),
            new Class<?>[]{service}, (proxy, method, values) -> {
                if (method.getReturnType() == int.class) return method.getName().equals("trackPlayer") ? 1 : 0;
                if (method.getReturnType() == boolean.class) return false;
                return null;
            });
        Object realAudio = Class.forName("android.os.ServiceManager")
            .getMethod("checkService",String.class).invoke(null,"audio");
        if (realAudio == null) {
            java.lang.reflect.Field field = Class.forName("android.media.PlayerBase").getDeclaredField("sService");
            field.setAccessible(true); field.set(null, bookkeeping);
            Class<?> system = Class.forName("android.media.AudioSystem");
            int initialized = (Integer)system.getMethod("initStreamVolume",int.class,int.class,int.class)
                .invoke(null,3,0,100);
            java.lang.reflect.Method setVolume = null;
            for (java.lang.reflect.Method method : system.getDeclaredMethods()) {
                if (method.getName().startsWith("setStreamVolumeIndex")) {
                    System.out.println("Volume API: "+method);
                    Class<?>[] parameters = method.getParameterTypes();
                    if (parameters.length == 3 && parameters[0] == int.class
                        && parameters[1] == int.class && parameters[2] == int.class) setVolume = method;
                }
            }
            if (setVolume == null) throw new IllegalStateException("No native stream-volume API");
            setVolume.setAccessible(true);
            int volume = (Integer)setVolume.invoke(null,3,30,2);
            if (initialized != 0 || volume != 0)
                throw new IllegalStateException("Could not apply native media volume 30/100: "+initialized+","+volume);
            System.out.println("Native media volume set to 30/100");
        }
    }

}
