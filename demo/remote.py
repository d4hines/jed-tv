#!/usr/bin/env python3
"""On-screen TCL remote for the PID-1 demo.

It has the buttons of the handset shipped with the 32Q3K (TCL G0401-000303
family): Power, Mute, a D-pad with OK, Back, Home, Volume +/-, Channel +/-,
NETFLIX, prime video and YouTube. No number pad, no colour keys, no play/pause,
fast-forward or rewind. Input, Settings and TCLtv+ are on the handset too but
their IR codes are unconfirmed on the device, so they are left out.

Each button sends the Linux keycode the TV's IR receiver emits for it (from the
firmware's ir_config.ini and Vendor_3697_Product_0001.kl; see
src/platform/src/remote.rs) as a datagram to PID 1's virtual IR receiver
(remote.udp=PORT on the kernel command line, reached through QEMU's user
network). A datagram holds 8-byte little-endian records: u16 type (1 = EV_KEY),
u16 code, i32 value (1 press, 2 repeat, 0 release). Holding a button repeats
like the real remote does."""
import socket,struct,sys,tkinter as tk

EV_SYN,EV_KEY=0,1
REPEAT_DELAY_MS,REPEAT_EVERY_MS,TAP_MS=400,100,80
# (label, Linux keycode)
POWER=('⏻',116);MUTE=('🔇',113)
UP=('▲',103);DOWN=('▼',108);LEFT=('◀',105);RIGHT=('▶',106);OK=('OK',28)
BACK=('Back',158);HOME=('Home',102)
VOLUP=('Vol +',115);VOLDOWN=('Vol −',114);CHUP=('Ch +',402);CHDOWN=('Ch −',403)
NETFLIX=('NETFLIX',566);PRIME=('prime video',752);YOUTUBE=('YouTube',569)
# Keys typed while the remote window has focus.
SHORTCUTS={'Up':UP,'Down':DOWN,'Left':LEFT,'Right':RIGHT,'Return':OK,'KP_Enter':OK,'Escape':BACK,'BackSpace':BACK,
           'Home':HOME,'plus':VOLUP,'equal':VOLUP,'minus':VOLDOWN,'Prior':CHUP,'Next':CHDOWN,'m':MUTE,
           'n':NETFLIX,'p':PRIME,'y':YOUTUBE}

class Ir:
 """Sends key records to the guest's virtual IR receiver."""
 def __init__(self,host,port):
  self.s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM);self.to=(host,port)
 def send(self,type_,code,value):
  self.s.sendto(struct.pack('<HHi',type_,code,value),self.to)
 def hello(self):
  # An empty event: lets QEMU's user network learn the guest's address before the first key.
  self.send(EV_SYN,0,0)

def main(host,port):
 ir=Ir(host,port)
 root=tk.Tk();root.title('TCL remote');root.configure(bg='#1b1d22',padx=14,pady=14);root.resizable(False,False)
 status=tk.StringVar(value=f'Keys go to the emulator at {host}:{port}')
 held={}  # keycode -> pending tk timer id while a button is held
 def send(key,value):
  label,code=key
  try:ir.send(EV_KEY,code,value)
  except OSError as e:status.set(f'{label}: {e}');return
  status.set(f'Sent {label} (Linux key {code}, {("release","press","repeat")[value]})')
 def repeat(key):
  send(key,2);held[key[1]]=root.after(REPEAT_EVERY_MS,repeat,key)
 def press(key,widget=None):
  send(key,1);held[key[1]]=root.after(REPEAT_DELAY_MS,repeat,key)
  if widget:widget.configure(bg='#5a6070')
 def release(key,widget=None):
  timer=held.pop(key[1],None)
  if timer is None:return
  root.after_cancel(timer);send(key,0)
  if widget:widget.configure(bg=widget.base)
 def tap(key):
  press(key);root.after(TAP_MS,release,key)
 def button(parent,key,row,col,bg='#30343d',fg='white',width=5,span=1,font=('Helvetica',15,'bold')):
  w=tk.Label(parent,text=key[0],bg=bg,fg=fg,width=width,height=2,font=font,cursor='hand2')
  w.base=bg;w.grid(row=row,column=col,columnspan=span,padx=3,pady=3,sticky='nsew')
  w.bind('<ButtonPress-1>',lambda e:press(key,w));w.bind('<ButtonRelease-1>',lambda e:release(key,w))
  return w
 def panel():
  f=tk.Frame(root,bg=root['bg']);f.pack(pady=6);return f
 top=panel();button(top,POWER,0,0,bg='#a12a2a');tk.Frame(top,width=70,bg=root['bg']).grid(row=0,column=1);button(top,MUTE,0,2)
 pad=panel()
 button(pad,UP,0,1);button(pad,LEFT,1,0);button(pad,OK,1,1,bg='#3d6fd1');button(pad,RIGHT,1,2);button(pad,DOWN,2,1)
 nav=panel();button(nav,BACK,0,0);tk.Frame(nav,width=40,bg=root['bg']).grid(row=0,column=1);button(nav,HOME,0,2)
 rockers=panel()
 button(rockers,VOLUP,0,0);button(rockers,CHUP,0,1)
 button(rockers,VOLDOWN,1,0);button(rockers,CHDOWN,1,1)
 apps=panel()
 button(apps,NETFLIX,0,0,bg='#e50914',width=9,font=('Helvetica',12,'bold'))
 button(apps,PRIME,0,1,bg='#00a8e1',width=9,font=('Helvetica',12,'bold'))
 button(apps,YOUTUBE,1,0,bg='#ffffff',fg='#ff0000',width=9,span=2,font=('Helvetica',12,'bold'))
 tk.Label(root,textvariable=status,bg=root['bg'],fg='#9aa0ab',font=('Helvetica',11)).pack(pady=(8,0))
 root.bind('<KeyPress>',lambda e:SHORTCUTS.get(e.keysym) and tap(SHORTCUTS[e.keysym]))
 try:ir.hello()
 except OSError as e:status.set(str(e))
 root.mainloop()

if __name__=='__main__':
 if len(sys.argv)!=2 or ':' not in sys.argv[1]:raise SystemExit('usage: remote.py HOST:PORT  (QEMU hostfwd to the guest virtual IR receiver)')
 host,port=sys.argv[1].rsplit(':',1)
 try:main(host,int(port))
 except KeyboardInterrupt:pass
