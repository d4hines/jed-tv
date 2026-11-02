//! Offline, byte-exact oracle for the production Lean compositor.
use g06_display::pixels::compose_pixels;
use std::io;

fn reference(
    dst: &mut [u32],
    dw: usize,
    dh: usize,
    stride: usize,
    src: &[u32],
    sw: usize,
    sh: usize,
    overlay: Option<&[u32]>,
) {
    let width = dw.min(dh * sw / sh);
    let height = dh.min(dw * sh / sw);
    let (left, top) = ((dw - width) / 2, (dh - height) / 2);
    for row in dst.chunks_mut(stride).take(dh) {
        row[..dw].fill(0xff000000);
    }
    for y in 0..height {
        for x in 0..width {
            dst[(top + y) * stride + left + x] = src[(y * sh / height) * sw + x * sw / width];
        }
    }
    if let Some(osd) = overlay {
        if dw >= 480 && dh >= 116 {
            for y in 0..88 {
                let start = (dh - 116 + y) * stride + (dw - 480) / 2;
                dst[start..start + 480].copy_from_slice(&osd[y * 480..(y + 1) * 480]);
            }
        }
    }
}

fn random(seed: &mut u32) -> u32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed
}

fn case(dw: usize, dh: usize, padding: usize, sw: usize, sh: usize, osd: bool, seed: u32) -> usize {
    let stride = dw + padding;
    let size = stride * dh;
    let mut state = seed;
    let src: Vec<_> = (0..sw * sh).map(|_| random(&mut state)).collect();
    let overlay: Vec<_> = (0..480 * 88).map(|_| random(&mut state)).collect();
    // Include guards and nonzero padding: a full-image match also checks that
    // the compositor neither writes outside the slice nor clears row padding.
    let mut actual: Vec<_> = (0..size + 32).map(|_| random(&mut state)).collect();
    let mut expected = actual.clone();
    let osd = osd.then_some(overlay.as_slice());
    compose_pixels(
        &mut actual[16..16 + size],
        dw,
        dh,
        stride,
        &src,
        sw,
        sh,
        osd,
    )
    .unwrap();
    reference(
        &mut expected[16..16 + size],
        dw,
        dh,
        stride,
        &src,
        sw,
        sh,
        osd,
    );
    assert_eq!(
        actual, expected,
        "output {dw}x{dh}/{stride}, source {sw}x{sh}"
    );
    size
}

fn malformed() {
    let source = [1, 2, 3, 4];
    let invalid = [
        (0, 2, 2, 2, 2),
        (2, 0, 2, 2, 2),
        (2, 2, 1, 2, 2),
        (2, 2, 2, 3, 2),
        (2, 2, 2, 0, 2),
        (2, 2, 2, 2, 0),
        (2, 2, 2, usize::MAX, 2),
        (1, usize::MAX, usize::MAX, 2, 2),
    ];
    for (dw, dh, stride, sw, sh) in invalid {
        let mut actual = [7; 16];
        assert!(compose_pixels(&mut actual, dw, dh, stride, &source, sw, sh, None).is_err());
        assert_eq!(actual, [7; 16]);
    }
    for len in [0, 1, 480 * 88 - 1, 480 * 88 + 1] {
        let mut actual = [7; 16];
        assert!(compose_pixels(&mut actual, 2, 2, 2, &source, 2, 2, Some(&vec![0; len])).is_err());
        assert_eq!(actual, [7; 16]);
    }
}

fn main() -> io::Result<()> {
    let mut seed = 0x86c125ef;
    let mut pixels = 0;
    for _ in 0..2000 {
        let dw = 1 + random(&mut seed) as usize % 96;
        let dh = 1 + random(&mut seed) as usize % 80;
        let sw = 1 + random(&mut seed) as usize % 112;
        let sh = 1 + random(&mut seed) as usize % 88;
        let padding = random(&mut seed) as usize % 8;
        pixels += case(dw, dh, padding, sw, sh, false, seed);
    }
    for &(dw, dh) in &[(480, 116), (479, 116), (480, 115), (640, 360), (1280, 720)] {
        for &(sw, sh) in &[(1, 1), (320, 200), (640, 480), (1280, 720)] {
            pixels += case(dw, dh, 7, sw, sh, true, seed);
        }
    }
    malformed();
    // Concurrent callers share no Lean allocator, initialization or globals.
    let threads: Vec<_> = (0..4)
        .map(|i| {
            std::thread::spawn(move || {
                for n in 0..16 {
                    case(64 + n, 48, 3, 37, 29, false, i * 16 + n as u32);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
    println!("PASS: 2084 byte-exact display cases, {pixels} main-thread pixels, invalid inputs, guards/padding, four concurrent callers");
    Ok(())
}
