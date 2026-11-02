import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Proxy;

/** Fixed, bounded diagnostic reads; no settings, shell commands or block writes. */
public final class G13FactoryReadProbe {
    private static Throwable cause(Throwable t) {
        return t instanceof InvocationTargetException ? ((InvocationTargetException)t).getCause() : t;
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 0) throw new IllegalArgumentException("No arguments accepted");
        Class<?> properties = Class.forName("android.os.SystemProperties");
        String fingerprint = (String)properties.getMethod("get", String.class)
            .invoke(null, "ro.build.fingerprint");
        System.out.println("fingerprint=" + fingerprint);
        if (!"TCL/G13_2K_US/G13:14/UKN2.241110.001/AU13:user/release-keys".equals(fingerprint))
            throw new SecurityException("Unexpected target firmware");
        ClassLoader loader = G13FactoryReadProbe.class.getClassLoader();
        for (String name : new String[]{"com.tcl.tosapi.atv.TvPlayerApi", "com.tcl.tosapi.atv.TFactoryApi"}) {
            try {
                Class.forName(name, false, loader);
                System.out.println(name + "=present");
            } catch (ClassNotFoundException absent) {
                System.out.println(name + "=absent");
            }
        }
        Class<?> api = Class.forName("vendor.mediatek.tv.mtktvfactory.V1_0.IMtkTvFApiSystem");
        Object service;
        try {
            service = api.getMethod("getService", String.class, boolean.class)
                .invoke(null, "default", false);
        } catch (Throwable t) {
            System.out.println("getServiceFailure=" + cause(t));
            System.exit(2);
            return;
        }
        System.out.println("factoryServiceAvailable=" + (service != null));
        if (service == null) System.exit(2);
        Class<?> callback = Class.forName(api.getName() + "$a_hidl_a_mtktvfapi_sys_read_fileCallback");
        String[] paths = {"/proc/self/status", "/proc/self/attr/current", "/vendor/tvconfig/model_name.bin"};
        int[] lengths = {4096, 256, 256};
        for (int index = 0; index < paths.length; index++) {
            final String path = paths[index];
            Object receiver = Proxy.newProxyInstance(callback.getClassLoader(), new Class<?>[]{callback},
                (proxy, method, values) -> {
                    if (method.getName().equals("onValues")) {
                        String value = (String)values[0];
                        System.out.println("READ " + path + " status=" + values[1]);
                        if (path.equals("/proc/self/status")) {
                            for (String line : value.split("\n"))
                                if (line.matches("^(Name|Pid|PPid|Uid|Gid|CapEff|NoNewPrivs):.*"))
                                    System.out.println(line);
                        } else {
                            System.out.println(value.replace("\u0000", ""));
                        }
                    }
                    return null;
                });
            try {
                api.getMethod("a_hidl_a_mtktvfapi_sys_read_file", String.class, int.class, long.class, callback)
                    .invoke(service, path, lengths[index], 0L, receiver);
            } catch (Throwable t) {
                System.out.println("ReadFailure " + path + " " + cause(t));
            }
        }
    }
}
