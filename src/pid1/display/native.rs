//! V109 Android 11 and G13 V013 Android 14 SurfaceFlinger ABIs (ARM32).
//! Private C++ calls are isolated here; no Java/ART is used.
use std::ffi::{c_char,c_void,CStr,CString};
use std::io;
pub type Ptr = *mut c_void;
extern "C" { fn dlopen(path:*const c_char,flags:i32)->Ptr; fn dlsym(lib:Ptr,name:*const c_char)->Ptr; fn dlerror()->*const c_char; }
fn error(s: impl Into<String>)->io::Error {io::Error::other(s.into())}
unsafe fn symbol<T:Copy>(lib:Ptr,name:&str)->io::Result<T>{
 let text=CString::new(name).unwrap();let p=dlsym(lib,text.as_ptr());
 if p.is_null(){return Err(error(format!("missing {name}")))}
 Ok(std::mem::transmute_copy(&p))
}
unsafe fn library(name:&CStr)->io::Result<Ptr>{let p=dlopen(name.as_ptr(),2);if p.is_null(){let e=dlerror();return Err(error(if e.is_null(){"dlopen failed".into()}else{CStr::from_ptr(e).to_string_lossy().into_owned()}))}Ok(p)}
type StrongDrop=unsafe extern "C" fn(Ptr,*const c_void);
type OnSurface=unsafe extern "C" fn(Ptr,*const Ptr)->Ptr;
#[repr(C)]
pub struct Buffer {pub width:i32,pub height:i32,pub stride:i32,pub format:i32,pub bits:Ptr,pub reserved:[u32;6]}
pub struct Api {
 _process:Strong,
 get_default:unsafe extern "C" fn(*mut Ptr),
 init_check:unsafe extern "C" fn(Ptr)->i32,
 dec_strong:StrongDrop,
 string_init:unsafe extern "C" fn(Ptr,*const c_char),string_drop:unsafe extern "C" fn(Ptr),
 metadata_init:unsafe extern "C" fn(Ptr),
 create:unsafe extern "C" fn(Ptr,Ptr,u32,u32,i32,*mut Ptr,u32,Ptr,Ptr,*mut u32)->i32,
 binder_parent:bool,
 get_surface:unsafe extern "C" fn(*mut Ptr,Ptr),
 lock:unsafe extern "C" fn(Ptr,*mut Buffer,Ptr)->i32,
 post:unsafe extern "C" fn(Ptr)->i32,
 transaction_create:unsafe extern "C" fn()->Ptr,
 transaction_drop:unsafe extern "C" fn(Ptr),
 transaction_apply:unsafe extern "C" fn(Ptr),
 show:OnSurface,hide:OnSurface,
 layer:unsafe extern "C" fn(Ptr,*const Ptr,i32)->Ptr,
 stack:unsafe extern "C" fn(Ptr,*const Ptr,u32)->Ptr,
}
pub struct Strong {slot:Box<Ptr>,drop:StrongDrop,virtual_base:bool}
impl Strong {
 fn empty(drop:StrongDrop)->Self{Self{slot:Box::new(std::ptr::null_mut()),drop,virtual_base:false}}
 pub fn get(&self)->Ptr{*self.slot}
 fn address(&self)->*const Ptr{&*self.slot}
}
impl Drop for Strong {fn drop(&mut self){if !self.get().is_null(){unsafe{
 let mut base=self.get();
 // ProcessState virtually inherits RefBase. Exact libgui callers load the
 // vbase adjustment from vtable[-3] before RefBase::decStrong (0x88660).
 if self.virtual_base {let table=*(base as *const *const isize);let offset=*table.offset(-3);base=(base as *mut u8).offset(offset).cast();}
 (self.drop)(base,self.address().cast());
}}}}
impl Api {
 pub fn requires_immediate_exit(&self)->bool {self.binder_parent}
 pub fn load()->io::Result<Self>{unsafe{
  let gui=library(c"libgui.so")?;let android=library(c"libandroid.so")?;
  let dec_strong:StrongDrop=symbol(gui,"_ZNK7android7RefBase9decStrongEPKv")?;
  let process_get:unsafe extern "C" fn(*mut Ptr)=symbol(gui,"_ZN7android12ProcessState4selfEv")?;
  let process_start:unsafe extern "C" fn(Ptr)=symbol(gui,"_ZN7android12ProcessState15startThreadPoolEv")?;
  let mut process=Strong::empty(dec_strong);process.virtual_base=true;process_get(&mut *process.slot);
  if process.get().is_null(){return Err(error("null Binder ProcessState"))}
  process_start(process.get());
  let old_create=symbol(gui,"_ZN7android21SurfaceComposerClient20createSurfaceCheckedERKNS_7String8EjjiPNS_2spINS_14SurfaceControlEEEjPS5_NS_13LayerMetadataEPj");
  let binder_parent=old_create.is_err();
  let create=old_create.or_else(|_|symbol(gui,"_ZN7android21SurfaceComposerClient20createSurfaceCheckedERKNS_7String8EjjiPNS_2spINS_14SurfaceControlEEEiRKNS4_INS_7IBinderEEENS_3gui13LayerMetadataEPj"))?;
  Ok(Self{_process:process,
   get_default:symbol(gui,"_ZN7android21SurfaceComposerClient10getDefaultEv")?,
   init_check:symbol(gui,"_ZNK7android21SurfaceComposerClient9initCheckEv")?,
   dec_strong:symbol(gui,"_ZNK7android7RefBase9decStrongEPKv")?,
   string_init:symbol(gui,"_ZN7android7String8C1EPKc")?,string_drop:symbol(gui,"_ZN7android7String8D1Ev")?,
   metadata_init:symbol(gui,if binder_parent{"_ZN7android3gui13LayerMetadataC1Ev"}else{"_ZN7android13LayerMetadataC1Ev"})?,
   create,binder_parent,
   get_surface:symbol(gui,if binder_parent{"_ZN7android14SurfaceControl10getSurfaceEv"}else{"_ZNK7android14SurfaceControl10getSurfaceEv"})?,
   lock:symbol(gui,"_ZN7android7Surface4lockEP20ANativeWindow_BufferP5ARect")?,
   post:symbol(gui,"_ZN7android7Surface13unlockAndPostEv")?,
   transaction_create:symbol(android,"ASurfaceTransaction_create")?,transaction_drop:symbol(android,"ASurfaceTransaction_delete")?,transaction_apply:symbol(android,"ASurfaceTransaction_apply")?,
   show:symbol(gui,"_ZN7android21SurfaceComposerClient11Transaction4showERKNS_2spINS_14SurfaceControlEEE")?,
   hide:symbol(gui,"_ZN7android21SurfaceComposerClient11Transaction4hideERKNS_2spINS_14SurfaceControlEEE")?,
   layer:symbol(gui,"_ZN7android21SurfaceComposerClient11Transaction8setLayerERKNS_2spINS_14SurfaceControlEEEi")?,
   stack:symbol(gui,if binder_parent{"_ZN7android21SurfaceComposerClient11Transaction13setLayerStackERKNS_2spINS_14SurfaceControlEEENS_2ui10LayerStackE"}else{"_ZN7android21SurfaceComposerClient11Transaction13setLayerStackERKNS_2spINS_14SurfaceControlEEEj"})?,
  })
 }}
 pub fn client(&self)->io::Result<Strong>{let mut client=Strong::empty(self.dec_strong);unsafe{(self.get_default)(&mut *client.slot);}
  if client.get().is_null(){return Err(error("null default composer client"))}
  let status=unsafe{(self.init_check)(client.get())};if status!=0{return Err(error(format!("composer initCheck {status}")))}Ok(client)
 }
 pub fn surface(&self,client:&Strong,name:&str,width:u32,height:u32)->io::Result<Strong>{unsafe{
  let name=CString::new(name).unwrap();let mut string8=0usize;
  (self.string_init)((&mut string8 as *mut usize).cast(),name.as_ptr());
  // Both exact ARM32 constructors write vptr + 20-byte libc++ unordered_map.
  let mut metadata=[0u32;6];(self.metadata_init)(metadata.as_mut_ptr().cast());
  let mut out=Strong::empty(self.dec_strong);
  // eHidden=4: no compositor visibility until show() is explicitly called.
  // Android 14 takes const sp<IBinder>&; Android 11 takes SurfaceControl*.
  let mut parent:Ptr=std::ptr::null_mut();
  let parent_arg=if self.binder_parent{(&mut parent as *mut Ptr).cast()}else{std::ptr::null_mut()};
  let status=(self.create)(client.get(),(&mut string8 as *mut usize).cast(),width,height,1,&mut *out.slot,4,parent_arg,metadata.as_mut_ptr().cast(),std::ptr::null_mut());
  // The default metadata owns no entries; call its virtual complete destructor.
  let table=metadata[0] as *const Ptr;
  let destructor:unsafe extern "C" fn(Ptr)=std::mem::transmute(*table);
  destructor(metadata.as_mut_ptr().cast());
  (self.string_drop)((&mut string8 as *mut usize).cast());
  if status!=0||out.get().is_null(){return Err(error(format!("createSurfaceChecked {status}")))}Ok(out)
 }}
 pub fn window(&self,surface:&Strong)->io::Result<Strong>{let mut out=Strong::empty(self.dec_strong);unsafe{(self.get_surface)(&mut *out.slot,surface.get())};if out.get().is_null(){Err(error("null Surface"))}else{Ok(out)}}
 pub fn visible(&self,surface:&Strong,show:bool)->io::Result<()>{unsafe{let t=(self.transaction_create)();if t.is_null(){return Err(error("null transaction"))}
  (self.stack)(t,surface.address(),0);(self.layer)(t,surface.address(),1000001);
  (if show{self.show}else{self.hide})(t,surface.address());(self.transaction_apply)(t);(self.transaction_drop)(t);Ok(())}}
 pub fn draw(&self,window:&Strong,draw:impl FnOnce(&mut Buffer)->io::Result<()>)->io::Result<()>{unsafe{
  let mut buffer:Buffer=std::mem::zeroed();let result=(self.lock)(window.get(),&mut buffer,std::ptr::null_mut());if result!=0{return Err(error(format!("Surface::lock {result}")))}
  let drawn=draw(&mut buffer);let posted=(self.post)(window.get());drawn?;if posted!=0{return Err(error(format!("Surface::unlockAndPost {posted}")))}Ok(())
 }}
}
