package local.tvreadback;

import android.app.Activity;
import android.os.Bundle;
import android.net.LocalServerSocket;
import android.net.LocalSocket;
import android.widget.TextView;
import java.io.*;
import java.lang.reflect.*;
import java.security.MessageDigest;
import java.util.*;

/** Read-only bridge to the installed HAL. No command-execution or write API. */
public final class ServerActivity extends Activity {
    private static final String EXPECTED="TCL/G13_2K_US/G13:14/UKN2.241110.001/AU13:user/release-keys";
    private static final int CHUNK=256*1024;
    private Object service;
    private Class<?> callback;
    private Method readMethod;
    private long rawLimit, boot0Size, boot1Size;
    private TextView view;
    private LocalServerSocket server;
    private PrintWriter log;
    private volatile boolean stopped;

    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        getWindow().addFlags(128); // Keep the foreground activity/screen awake.
        view=new TextView(this);
        view.setText("Preparing firmware backup. Keep this screen open.");
        setContentView(view);
        new Thread(()->serve(),"firmware-readback").start();
    }

    private byte[] read(String path,int count,long offset) throws Exception {
        final byte[][] result={null};
        final int[] status={Integer.MIN_VALUE};
        Object receiver=Proxy.newProxyInstance(callback.getClassLoader(),new Class<?>[]{callback},
            (proxy,method,args)->{
                if(method.getName().equals("onValues")) {
                    List<?> vector=(List<?>)args[0];
                    byte[] data=new byte[vector.size()];
                    for(int i=0;i<data.length;i++) data[i]=(Byte)vector.get(i);
                    result[0]=data;
                    status[0]=(Integer)args[1];
                }
                return null;
            });
        readMethod.invoke(service,path,count,offset,receiver);
        if(status[0]!=0 || result[0]==null) throw new IOException("HAL status="+status[0]+" path="+path);
        return result[0];
    }

    private long sectors(String node,String key) throws Exception {
        return Long.parseLong(new String(read("/sys/class/block/"+node+"/"+key,128,0),"UTF-8").trim());
    }
    private static String hex(byte[] bytes) {
        StringBuilder b=new StringBuilder();
        for(byte x:bytes) b.append(String.format(Locale.US,"%02x",x&255));
        return b.toString();
    }
    private void line(OutputStream out,String value) throws IOException {
        out.write((value+"\n").getBytes("UTF-8")); out.flush();
    }
    private void serve() {
        try {
            log=new PrintWriter(new File(getFilesDir(),"bridge.log"));
            if(!EXPECTED.equals(android.os.Build.FINGERPRINT)) throw new SecurityException("Unexpected firmware");
            Class<?> api=Class.forName("vendor.mediatek.tv.mtktvfactory.V1_0.IMtkTvFApiSystem");
            service=api.getMethod("getService",String.class,boolean.class).invoke(null,"default",false);
            if(service==null) throw new IOException("Factory service unavailable");
            callback=Class.forName(api.getName()+"$a_hidl_a_mtktvfapi_sys_read_file_byteCallback");
            readMethod=api.getMethod("a_hidl_a_mtktvfapi_sys_read_file_byte",String.class,int.class,long.class,callback);
            rawLimit=Math.multiplyExact(sectors("mmcblk0p51","start"),512L);
            boot0Size=Math.multiplyExact(sectors("mmcblk0boot0","size"),512L);
            boot1Size=Math.multiplyExact(sectors("mmcblk0boot1","size"),512L);
            String token=UUID.randomUUID().toString();
            String name="tvreadback-"+UUID.randomUUID().toString();
            server=new LocalServerSocket(name);
            // UUIDs and the fixed fingerprint contain no JSON quoting characters.
            String info="{\"socket\":\""+name+"\",\"token\":\""+token+"\",\"fingerprint\":\""+EXPECTED+
                "\",\"raw_limit_excluding_userdata\":"+rawLimit+",\"boot0_size\":"+boot0Size+
                ",\"boot1_size\":"+boot1Size+"}";
            try(PrintWriter f=new PrintWriter(new File(getFilesDir(),"server.json"))) { f.println(info); }
            // Authorized ADB can discover the ephemeral socket without run-as/debug JNI overhead.
            android.util.Log.i("TVReadback",info);
            runOnUiThread(()->view.setText("Firmware backup ready. Keep this screen open."));
            log.println(info); log.flush();
            while(!stopped) {
                LocalSocket socket=null;
                try {
                    socket=server.accept();
                    socket.setSoTimeout(300000);
                    BufferedReader in=new BufferedReader(new InputStreamReader(socket.getInputStream(),"UTF-8"));
                    OutputStream out=new BufferedOutputStream(socket.getOutputStream(),CHUNK);
                    if(!("AUTH "+token).equals(in.readLine())) { line(out,"ERROR authentication"); continue; }
                    line(out,"OK "+EXPECTED);
                    String request;
                    while((request=in.readLine())!=null) {
                        if(request.length()>300) throw new IOException("Oversized request");
                        String[] fields=request.split(" ");
                        if(fields.length==2 && fields[0].equals("TEXT")) {
                            String path=fields[1];
                            if(!path.matches("/sys/class/block/mmcblk0(p[1-9][0-9]?|boot[01])?/(start|size|uevent|ro)")
                                && !path.equals("/proc/partitions")) throw new SecurityException("Disallowed text path");
                            byte[] bytes=read(path,16384,0);
                            line(out,"DATA "+bytes.length); out.write(bytes); out.flush();
                        } else if(fields.length==4 && (fields[0].equals("READ") || fields[0].equals("HASH"))) {
                            String source=fields[1];
                            long offset=Long.parseLong(fields[2]), length=Long.parseLong(fields[3]);
                            long limit;
                            if(source.equals("/dev/block/mmcblk0")) limit=rawLimit;
                            else if(source.equals("/dev/block/mmcblk0boot0")) limit=boot0Size;
                            else if(source.equals("/dev/block/mmcblk0boot1")) limit=boot1Size;
                            else throw new SecurityException("Disallowed source");
                            if(offset<0 || length<=0 || length>64L*1024*1024 || offset>limit || length>limit-offset
                               || offset%512!=0 || length%512!=0) throw new SecurityException("Invalid read range");
                            boolean hashOnly=fields[0].equals("HASH");
                            log.println(request); log.flush();
                            line(out,(hashOnly?"HASHING ":"DATA ")+length);
                            MessageDigest digest=MessageDigest.getInstance("SHA-256");
                            for(long done=0;done<length;) {
                                int count=(int)Math.min(CHUNK,length-done);
                                byte[] data=read(source,count,offset+done);
                                if(data.length!=count) throw new IOException("Short read: "+data.length+" != "+count);
                                digest.update(data);
                                if(!hashOnly) out.write(data);
                                done+=count;
                            }
                            line(out,"SHA256 "+hex(digest.digest()));
                        } else throw new SecurityException("Unknown command");
                    }
                } catch(Throwable e) { e.printStackTrace(log); log.flush(); }
                finally { if(socket!=null) try { socket.close(); } catch(IOException ignored) {} }
            }
        } catch(Throwable e) {
            try(PrintWriter failure=new PrintWriter(new File(getFilesDir(),"server-failure.txt"))) {
                (e instanceof InvocationTargetException?e.getCause():e).printStackTrace(failure);
            } catch(Exception ignored) {}
            runOnUiThread(()->view.setText("Backup stopped. Check the laptop for details."));
        }
    }
    @Override public void onDestroy() {
        stopped=true;
        try { if(server!=null) server.close(); } catch(IOException ignored) {}
        super.onDestroy();
    }
}
