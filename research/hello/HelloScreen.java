import java.lang.reflect.Constructor;
import java.lang.reflect.Method;

/** A bounded shell-owned SurfaceFlinger layer; stock Android and ADB stay running. */
public final class HelloScreen {
    private static Object call(Object object, String name, Class<?>[] types, Object... args) throws Exception {
        return object.getClass().getMethod(name, types).invoke(object, args);
    }

    private static void event(String message) throws Exception {
        System.out.println("TVHELLO " + message);
        System.out.flush();
        Class.forName("android.util.Log").getMethod("i", String.class, String.class)
            .invoke(null, "TVHello", message);
    }

    private static void inspect() throws Exception {
        for (String name : new String[] {"android.view.SurfaceControl$Builder", "android.view.SurfaceControl$Transaction",
                                        "android.view.Surface", "android.view.SurfaceSession"}) {
            Class<?> type = Class.forName(name);
            System.out.println("CLASS " + name);
            for (Constructor<?> constructor : type.getConstructors()) System.out.println(constructor);
            for (Method method : type.getMethods()) {
                if (method.getDeclaringClass() == type) System.out.println(method);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        if (args.length == 1 && args[0].equals("--inspect")) { inspect(); return; }
        int seconds = args.length == 0 ? 20 : Integer.parseInt(args[0]);
        if (seconds < 1 || seconds > 120) throw new IllegalArgumentException("Duration must be 1..120 seconds");
        Thread deadline = new Thread(() -> {
            try { Thread.sleep((seconds + 10L) * 1000L); }
            catch (InterruptedException interrupted) { return; }
            System.err.println("TVHELLO phase=deadline-exceeded");
            Runtime.getRuntime().halt(124);
        }, "hello-deadline");
        deadline.setDaemon(true);
        deadline.start();
        int pid = (Integer) Class.forName("android.os.Process").getMethod("myPid").invoke(null);
        event("pid=" + pid + " phase=start");
        Class<?> controlClass = Class.forName("android.view.SurfaceControl");
        Class<?> sessionClass = Class.forName("android.view.SurfaceSession");
        Class<?> transactionClass = Class.forName("android.view.SurfaceControl$Transaction");
        Object session = sessionClass.getConstructor().newInstance();
        Object control = null;
        Object surface = null;
        try {
            Object builder = Class.forName("android.view.SurfaceControl$Builder")
                .getConstructor(sessionClass).newInstance(session);
            call(builder, "setName", new Class<?>[] {String.class}, "dande-tv Hello world");
            call(builder, "setBufferSize", new Class<?>[] {int.class, int.class}, 1280, 720);
            call(builder, "setFormat", new Class<?>[] {int.class}, 1);
            control = call(builder, "build", new Class<?>[] {});
            event("phase=surface-created width=1280 height=720");
            Class<?> surfaceClass = Class.forName("android.view.Surface");
            surface = surfaceClass.getConstructor(controlClass).newInstance(control);
            Class<?> rectClass = Class.forName("android.graphics.Rect");
            Object canvas = call(surface, "lockCanvas", new Class<?>[] {rectClass}, new Object[] {null});
            Class<?> canvasClass = Class.forName("android.graphics.Canvas");
            Class<?> paintClass = Class.forName("android.graphics.Paint");
            Object paint = paintClass.getConstructor().newInstance();
            call(paint, "setAntiAlias", new Class<?>[] {boolean.class}, true);
            call(paint, "setColor", new Class<?>[] {int.class}, 0xfff4f6fa);
            canvasClass.getMethod("drawColor", int.class).invoke(canvas, 0xff102030);
            int[][] glyphs = {
                {17,17,17,31,17,17,17}, {0,0,14,17,31,16,14},
                {12,4,4,4,4,4,14}, {12,4,4,4,4,4,14}, {0,0,14,17,17,17,14},
                {0,0,0,0,0,0,0}, {0,0,17,17,21,21,10}, {0,0,14,17,17,17,14},
                {0,0,22,25,16,16,16}, {12,4,4,4,4,4,14}, {1,1,15,17,17,17,15}
            };
            Method rectangle = canvasClass.getMethod("drawRect", float.class, float.class, float.class, float.class, paintClass);
            int scale = 14;
            int left = (1280 - (glyphs.length * 6 - 1) * scale) / 2;
            int top = (720 - 7 * scale) / 2;
            for (int letter = 0; letter < glyphs.length; letter++) {
                for (int row = 0; row < 7; row++) {
                    for (int column = 0; column < 5; column++) {
                        if ((glyphs[letter][row] & (1 << (4 - column))) == 0) continue;
                        float x = left + (letter * 6 + column) * scale;
                        float y = top + row * scale;
                        rectangle.invoke(canvas, x, y, x + scale, y + scale, paint);
                    }
                }
            }
            call(surface, "unlockCanvasAndPost", new Class<?>[] {canvasClass}, canvas);
            Object transaction = transactionClass.getConstructor().newInstance();
            try {
                call(transaction, "setLayerStack", new Class<?>[] {controlClass, int.class}, control, 0);
                call(transaction, "setLayer", new Class<?>[] {controlClass, int.class}, control, 1000000);
                call(transaction, "show", new Class<?>[] {controlClass}, control);
                call(transaction, "apply", new Class<?>[] {});
            } finally { call(transaction, "close", new Class<?>[] {}); }
            event("phase=visible seconds=" + seconds);
            Thread.sleep(seconds * 1000L);
        } finally {
            if (control != null) {
                Object transaction = transactionClass.getConstructor().newInstance();
                try {
                    call(transaction, "remove", new Class<?>[] {controlClass}, control);
                    call(transaction, "apply", new Class<?>[] {});
                } finally { call(transaction, "close", new Class<?>[] {}); }
                call(control, "release", new Class<?>[] {});
            }
            if (surface != null) call(surface, "release", new Class<?>[] {});
            call(session, "kill", new Class<?>[] {});
            event("phase=closed");
        }
    }
}
