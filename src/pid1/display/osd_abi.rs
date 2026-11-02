//! V109 direct OSD/MMA wire constructors. No device access in this module.
//! Evidence and lifetime constraints: research/DIRECT_OSD_WITHOUT_FBDEV.md.

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
pub const PITCH: u32 = WIDTH * 4;
pub const PAGE_BYTES: u32 = PITCH * HEIGHT;
pub const ALLOCATION_BYTES: u32 = PAGE_BYTES * 2;
pub const MMA_ALLOC: u32 = 0xc0304d03;
pub const LAYER_CREATE: u32 = 0xc0241403;
pub const SURFACE_CREATE: u32 = 0xc038142f;
pub const WINDOW_CREATE: u32 = 0xc030141b;
pub const WINDOW_FLIP_EXTERNAL: u32 = 0xc008142d;

/// DMA-BUF allocation, not the distinct 40-byte MMA control operation.
/// The kernel returns IOVA at byte24 and owned descriptor at byte36.
pub fn allocation() -> [u32; 12] {
    let mut wire = [0; 12];
    wire[0] = u32::from_le_bytes(*b"fbde");
    wire[1] = u32::from_le_bytes([b'v', 0, 0, 0]);
    wire[8] = ALLOCATION_BYTES;
    wire[9] = u32::MAX;
    wire
}

pub fn allocation_result(wire: &[u32; 12]) -> Result<(u64, i32), &'static str> {
    let address = wire[6] as u64 | ((wire[7] as u64) << 32);
    if wire[8] != ALLOCATION_BYTES || wire[9] > i32::MAX as u32 {
        return Err("invalid MMA allocation length or descriptor");
    }
    if address < 0x200000000 || address.checked_add(ALLOCATION_BYTES as u64).is_none() {
        return Err("invalid V109 full-width allocation IOVA");
    }
    Ok((address, wire[9] as i32))
}

pub fn layer(id: u32) -> Result<[u32; 9], &'static str> {
    if id > 1 {
        return Err("only reviewed cold layer0 or independent layer1 are supported");
    }
    let mut wire = [0; 9];
    wire[1] = id;
    // Exact working fbdev layout: 720p source, 1080p display timing space.
    wire[2] = 7; // CUSTOM
    wire[5] = WIDTH;
    wire[6] = HEIGHT;
    wire[7] = 1920;
    wire[8] = 1080;
    Ok(wire)
}

pub fn surface(address: u64, page: u32) -> Result<[u32; 14], &'static str> {
    if page >= 2 || address < 0x200000000 {
        return Err("invalid page or full-width IOVA");
    }
    // Check the entire backing allocation, not just this page's start.
    address
        .checked_add(ALLOCATION_BYTES as u64)
        .ok_or("IOVA overflow")?;
    let page_address = address + PAGE_BYTES as u64 * page as u64;
    let mut wire = [0; 14];
    wire[2] = WIDTH;
    wire[3] = HEIGHT;
    wire[4] = PITCH;
    wire[5] = 8; // ARGB8888; CPU bytes are BGRA.
    wire[6] = 0; // AP owns backing storage.
    wire[7] = 2; // PHY_OS; no userspace pointer translation.
    wire[8] = page_address as u32;
    wire[9] = (page_address >> 32) as u32;
    Ok(wire)
}

pub fn window(layer_handle: u32) -> Result<[u32; 12], &'static str> {
    if layer_handle != 0x20000 && layer_handle != 0x20001 {
        return Err("unexpected V109 layer handle");
    }
    let mut wire = [0; 12];
    wire[1] = layer_handle;
    wire[4] = WIDTH;
    wire[5] = HEIGHT;
    wire[6] = 1; // Pixel alpha, followed by WindowSetAlpha(enable=true, alpha=255).
    wire[7] = 8; // ARGB8888.
    wire[8] = 2; // EXTERNAL; kernel must not allocate window pages.
    wire[9] = WIDTH;
    wire[10] = HEIGHT;
    wire[11] = 0; // Match fbdev's non-full-flip window setting.
    Ok(wire)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocation_is_nonsecure_and_has_invalid_fd_sentinel() {
        let w = allocation();
        assert_eq!(std::mem::size_of_val(&w), 48);
        assert_eq!(w[0].to_le_bytes(), *b"fbde");
        assert_eq!(w[1].to_le_bytes(), [b'v', 0, 0, 0]);
        assert_eq!(w[8], 7_372_800);
        assert_eq!(w[9], u32::MAX);
        assert_eq!(w[10], 0);
        assert!(allocation_result(&w).is_err());
    }
    #[test]
    fn full_width_iova_survives_page_offset() {
        let mut w = allocation();
        w[6] = 0x00e53000;
        w[7] = 2;
        w[9] = 17;
        let (address, fd) = allocation_result(&w).unwrap();
        assert_eq!((address, fd), (0x200e53000, 17));
        let page = surface(address, 1).unwrap();
        assert_eq!(
            page[8] as u64 | ((page[9] as u64) << 32),
            address + PAGE_BYTES as u64
        );
        assert_eq!(std::mem::size_of_val(&page), 56);
        assert_eq!(page[6], 0);
        assert_eq!(page[7], 2);
        assert_eq!(page[12], 0);
        assert!(surface(address, 2).is_err());
        assert!(surface(u64::MAX - PAGE_BYTES as u64, 0).is_err());
        assert!(surface(address as u32 as u64, 0).is_err());
    }
    #[test]
    fn exact_layer_and_window_sizes_and_external_geometry() {
        let layer = layer(1).unwrap();
        assert_eq!(std::mem::size_of_val(&layer), 36);
        assert_eq!(layer, [0, 1, 7, 0, 0, 1280, 720, 1920, 1080]);
        let window = window(0x20001).unwrap();
        assert_eq!(std::mem::size_of_val(&window), 48);
        assert_eq!(window, [0, 0x20001, 0, 0, 1280, 720, 1, 8, 2, 1280, 720, 0]);
        assert!(super::layer(2).is_err());
        assert!(super::window(0x40000).is_err());
    }
}
