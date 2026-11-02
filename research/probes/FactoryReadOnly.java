import java.lang.reflect.Method;

/** Read-only inspection of the factory API from the owner's stock firmware. */
public final class FactoryReadOnly {
    public static void main(String[] args) throws Exception {
        if (args.length != 0) {
            throw new IllegalArgumentException("This diagnostic accepts no commands or arguments");
        }
        // This stock API class performs the same JNI loading used by FactorySettings.
        Class.forName("com.tcl.tosapi.atv.TvPlayerApi");
        Class<?> factory = Class.forName("com.tcl.tosapi.atv.TFactoryApi");
        Object instance = factory.getMethod("getInstance").invoke(null);
        for (String name : new String[] { "doGetUartStatus", "doGetBootCodeVersion" }) {
            Method getter = factory.getMethod(name);
            System.out.println(name + "=" + getter.invoke(instance));
        }
    }
}
