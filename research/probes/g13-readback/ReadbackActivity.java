package local.tvreadback;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;
import java.io.File;
import java.io.FileOutputStream;
import java.io.PrintWriter;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Proxy;
import java.util.List;

/** Fixed diagnostic reads through the installed vendor API. No write API exists here. */
public final class ReadbackActivity extends Activity {
    private Object service;
    private Class<?> api;

    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        TextView view = new TextView(this);
        view.setText("Checking firmware read access...");
        setContentView(view);
        new Thread(() -> {
            try (PrintWriter out = new PrintWriter(new File(getFilesDir(), "probe.txt"))) {
                out.println("uid=" + android.os.Process.myUid());
                String fingerprint = android.os.Build.FINGERPRINT;
                out.println("fingerprint=" + fingerprint);
                if (!fingerprint.equals("TCL/G13_2K_US/G13:14/UKN2.241110.001/AU13:user/release-keys"))
                    throw new SecurityException("Unexpected firmware");
                api = Class.forName("vendor.mediatek.tv.mtktvfactory.V1_0.IMtkTvFApiSystem");
                service = api.getMethod("getService", String.class, boolean.class).invoke(null,"default",false);
                out.println("service_available=" + (service != null));
                if (service == null) throw new IllegalStateException("No factory service");
                String[] paths={"/proc/self/status","/proc/self/attr/current",
                    "/sys/class/block/mmcblk0/size","/sys/class/block/mmcblk0p1/start",
                    "/dev/block/mmcblk0","/dev/block/mmcblk0boot0","/dev/block/by-name/MBOOT"};
                for (int i=0;i<paths.length;i++) {
                    final String path=paths[i];
                    final int index=i;
                    int count=i<4 ? 4096 : 512;
                    Class<?> callback=Class.forName(api.getName()+"$a_hidl_a_mtktvfapi_sys_read_file_byteCallback");
                    Object receiver=Proxy.newProxyInstance(callback.getClassLoader(),new Class<?>[]{callback},
                        (proxy,method,args)->{
                            if (method.getName().equals("onValues")) {
                                List<?> values=(List<?>)args[0];
                                byte[] bytes=new byte[values.size()];
                                for(int j=0;j<bytes.length;j++) bytes[j]=(Byte)values.get(j);
                                out.println("READ "+path+" status="+args[1]+" bytes="+bytes.length);
                                if(index<4) out.println(new String(bytes,"UTF-8").replace("\u0000",""));
                                else try(FileOutputStream f=new FileOutputStream(new File(getFilesDir(),"header-"+index+".bin"))) {
                                    f.write(bytes);
                                }
                                out.flush();
                            }
                            return null;
                        });
                    try {
                        api.getMethod("a_hidl_a_mtktvfapi_sys_read_file_byte",String.class,int.class,long.class,callback)
                           .invoke(service,path,count,0L,receiver);
                    } catch(Throwable error) {
                        out.println("READ_FAILURE "+path);
                        (error instanceof InvocationTargetException ? error.getCause():error).printStackTrace(out);
                    }
                    out.flush();
                }
                out.println("DONE");
            } catch(Throwable error) {
                try(PrintWriter out=new PrintWriter(new FileOutputStream(new File(getFilesDir(),"failure.txt")))) {
                    (error instanceof InvocationTargetException ? error.getCause():error).printStackTrace(out);
                } catch(Exception ignored) {}
            }
            runOnUiThread(()->view.setText("Read-access check finished. Results saved locally."));
        },"readback-probe").start();
    }
}
