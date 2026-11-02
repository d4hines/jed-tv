//! Software composition while SurfaceFlinger remains the presentation backend.
use crate::native::Buffer;
use std::io;
use crate::pixels::compose_pixels;
pub fn compose(buffer:&mut Buffer,pixels:&[u32],width:usize,height:usize,overlay:Option<&[u32]>)->io::Result<()> {
 if buffer.width<=0||buffer.height<=0||buffer.stride<buffer.width||buffer.bits.is_null()||buffer.format!=1{return Err(io::Error::other("unsupported native buffer"))}
 let (dw,dh,stride)=(buffer.width as usize,buffer.height as usize,buffer.stride as usize);
 let dst=unsafe{std::slice::from_raw_parts_mut(buffer.bits.cast::<u32>(),stride*dh)};
 compose_pixels(dst,dw,dh,stride,pixels,width,height,overlay)
}
