//! Shared Rust rendering and publication for the system volume overlay.
#[path = "audio_level.rs"]
pub mod level;
const WIDTH:usize=480;
const HEIGHT:usize=88;

pub fn render(gain:f32)->Vec<u8> {
    let level=level::percent(gain) as usize;
    let mut pixels=vec![0xff241c16u32;WIDTH*HEIGHT];
    let text=if level==0 { "MUTED".into() } else { format!("VOLUME {level}%") };
    for (index,ch) in text.bytes().enumerate() {
        let glyph=font8x8::legacy::BASIC_LEGACY[ch as usize];
        for (row,bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if bits&(1<<col)!=0 {
                    for y in 0..3 { for x in 0..3 {
                        pixels[(16+row*3+y)*WIDTH+24+index*24+col*3+x]=0xfff5f5f5;
                    }}
                }
            }
        }
    }
    for y in 60..70 { for x in 24..456 {
        pixels[y*WIDTH+x]=if x<24+432*level/100 { 0xffa6d772 } else { 0xff544a42 };
    }}
    pixels.into_iter().flat_map(u32::to_le_bytes).collect()
}


pub fn publish(gain:f32) -> std::io::Result<u64> {
    let revision=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default().as_nanos() as u64;
    let level=level::percent(gain);
    let mut bytes=Vec::with_capacity(16+WIDTH*HEIGHT*4);
    bytes.extend_from_slice(&0x3144534fu32.to_le_bytes());
    bytes.extend_from_slice(&revision.to_le_bytes());
    bytes.extend_from_slice(&level.to_le_bytes());
    bytes.extend_from_slice(&render(gain));
    std::fs::write("/tmp/.app-volume-osd.rgba",bytes)?;
    std::fs::rename("/tmp/.app-volume-osd.rgba","/tmp/app-volume-osd.rgba")?;
    Ok(revision)
}
