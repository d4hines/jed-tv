//! Pure software composition shared by native and direct scanout.
use std::io;
pub fn compose_pixels(
    dst: &mut [u32],
    dw: usize,
    dh: usize,
    stride: usize,
    src: &[u32],
    sw: usize,
    sh: usize,
    overlay: Option<&[u32]>,
) -> io::Result<()> {
    let word = |n: usize| {
        u32::try_from(n).map_err(|_| io::Error::other("display buffer exceeds word bounds"))
    };
    let (overlay, overlay_len) = overlay.map_or((std::ptr::null(), 0), |p| (p.as_ptr(), p.len()));
    let result = unsafe {
        tv_display_compose(
            dst.as_mut_ptr(),
            word(dst.len())?,
            word(dw)?,
            word(dh)?,
            word(stride)?,
            src.as_ptr(),
            word(src.len())?,
            word(sw)?,
            word(sh)?,
            overlay,
            word(overlay_len)?,
        )
    };
    match result {
        0 => Ok(()),
        2 => Err(io::Error::other("scale overflow")),
        3 => Err(io::Error::other("invalid overlay")),
        _ => Err(io::Error::other("invalid pixels")),
    }
}

// Every pointer denotes a live slice for this synchronous call, and the
// exclusive destination cannot alias either immutable input. The Lean
// implementation validates all lengths before memory effects.
// Its linked entry is checked to have no allocator/runtime dependencies.
extern "C" {
    fn tv_display_compose(
        dst: *mut u32,
        dst_pixels: u32,
        dw: u32,
        dh: u32,
        stride: u32,
        src: *const u32,
        src_pixels: u32,
        sw: u32,
        sh: u32,
        overlay: *const u32,
        overlay_pixels: u32,
    ) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn letterbox_and_scale() {
        let mut dst = vec![0; 8 * 6];
        compose_pixels(&mut dst, 8, 6, 8, &[1, 2, 3, 4], 2, 2, None).unwrap();
        assert!(dst
            .chunks(8)
            .all(|r| r[0] == 0xff000000 && r[7] == 0xff000000));
        assert_eq!(dst[1], 1);
        assert_eq!(dst[4], 2);
        assert_eq!(dst[5 * 8 + 1], 3);
    }
    #[test]
    fn malformed_dimensions_return_errors_without_panics() {
        let mut dst = vec![7; 4];
        for (dw, dh, stride, sw, sh) in [
            (2, 2, 0, 2, 2),
            (3, 1, 2, 2, 2),
            (0, 2, 2, 2, 2),
            (2, 2, 2, usize::MAX, 2),
            (1, usize::MAX, usize::MAX, 2, 2),
        ] {
            assert!(compose_pixels(&mut dst, dw, dh, stride, &[1, 2, 3, 4], sw, sh, None).is_err());
            assert_eq!(dst, [7; 4]);
        }
    }
    #[test]
    fn exact_copy() {
        let mut dst = vec![0; 4];
        compose_pixels(&mut dst, 2, 2, 2, &[1, 2, 3, 4], 2, 2, None).unwrap();
        assert_eq!(dst, vec![1, 2, 3, 4]);
    }
}
