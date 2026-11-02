//! Software renderer: a 320x180 colour buffer (0x00RRGGBB) with a depth buffer,
//! a perspective camera, flat-coloured triangles clipped at the near plane,
//! and the 2D primitives the game's HUD uses. No GPU; it runs on the TV's CPU.

use font8x8::legacy::BASIC_LEGACY;

pub const W: usize = 320;
pub const H: usize = 180;
const NEAR: f32 = 0.05;
const FAR: f32 = 400.0;

type Vec3 = [f32; 3];
/// A vertex in clip space: x, y, z, w.
type Clip = [f32; 4];

pub struct Renderer {
    pub color: Vec<u32>,
    pub depth: Vec<f32>,
    view_proj: [[f32; 4]; 4],
    cam: Vec3,
}

#[inline]
fn blend(dst: u32, rgba: u32) -> u32 {
    let a = rgba & 255;
    if a >= 255 {
        return rgba >> 8;
    }
    let inv = 255 - a;
    let ch = |shift: u32| (((dst >> shift) & 255) * inv + ((rgba >> (shift + 8)) & 255) * a + 127) / 255;
    ch(16) << 16 | ch(8) << 8 | ch(0)
}

fn sub(a: Vec3, b: Vec3) -> Vec3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn cross(a: Vec3, b: Vec3) -> Vec3 { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn dot(a: Vec3, b: Vec3) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn norm(a: Vec3) -> Vec3 {
    let l = dot(a, a).sqrt();
    if l < 1e-6 { [0.0, 0.0, 1.0] } else { [a[0] / l, a[1] / l, a[2] / l] }
}

fn mul(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut m = [[0.0; 4]; 4];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..4).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    m
}

/// Faces of a unit cube at the origin, as in the game's `faceGeometry`:
/// normal, then four corners (x,y,z in {0,1}).
const CUBE_FACES: [([f32; 3], [[f32; 3]; 4]); 6] = [
    ([0.0, 1.0, 0.0], [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]]),
    ([0.0, -1.0, 0.0], [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]]),
    ([0.0, 0.0, -1.0], [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]]),
    ([0.0, 0.0, 1.0], [[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]]),
    ([-1.0, 0.0, 0.0], [[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]]),
    ([1.0, 0.0, 0.0], [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]]),
];

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { color: vec![0; W * H], depth: vec![f32::INFINITY; W * H], view_proj: [[0.0; 4]; 4], cam: [0.0; 3] }
    }

    pub fn begin_frame(&mut self) {
        self.color.fill(0);
        self.depth.fill(f32::INFINITY);
    }

    // ---------- 2D ----------

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, rgba: u32) {
        let (x0, x1) = (x.clamp(0, W as i32) as usize, (x + w).clamp(0, W as i32) as usize);
        let (y0, y1) = (y.clamp(0, H as i32) as usize, (y + h).clamp(0, H as i32) as usize);
        for row in y0..y1 {
            for p in &mut self.color[row * W + x0..row * W + x1] {
                *p = blend(*p, rgba);
            }
        }
    }

    /// Colour changes from `top` to `bottom` down the rectangle.
    pub fn gradient_v(&mut self, x: i32, y: i32, w: i32, h: i32, top: u32, bottom: u32) {
        for row in y.max(0)..(y + h).min(H as i32) {
            let t = (row - y) as f32 / h.max(1) as f32;
            self.rect(x, row, w, 1, lerp_rgba(top, bottom, t));
        }
    }

    /// Colour changes from `left` to `right` across the rectangle.
    pub fn gradient_h(&mut self, x: i32, y: i32, w: i32, h: i32, left: u32, right: u32) {
        for col in x.max(0)..(x + w).min(W as i32) {
            let t = (col - x) as f32 / w.max(1) as f32;
            self.rect(col, y, 1, h, lerp_rgba(left, right, t));
        }
    }

    pub fn circle(&mut self, cx: i32, cy: i32, r: f32, rgba: u32) {
        let ri = r.ceil() as i32;
        for dy in -ri..=ri {
            for dx in -ri..=ri {
                if (dx * dx + dy * dy) as f32 <= r * r {
                    let (x, y) = (cx + dx, cy + dy);
                    if (0..W as i32).contains(&x) && (0..H as i32).contains(&y) {
                        let p = &mut self.color[y as usize * W + x as usize];
                        *p = blend(*p, rgba);
                    }
                }
            }
        }
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, rgba: u32) {
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let (mut x, mut y, mut err) = (x0, y0, dx + dy);
        loop {
            if (0..W as i32).contains(&x) && (0..H as i32).contains(&y) {
                let p = &mut self.color[y as usize * W + x as usize];
                *p = blend(*p, rgba);
            }
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }

    /// 8x8 bitmap font at an integer scale.
    pub fn text(&mut self, x: i32, y: i32, scale: u32, rgba: u32, s: &str) {
        let scale = scale.max(1) as i32;
        for (i, ch) in s.chars().enumerate() {
            let glyph = BASIC_LEGACY.get(ch as usize).copied().unwrap_or([0; 8]);
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        self.rect(x + (i as i32 * 8 + col) * scale, y + row as i32 * scale, scale, scale, rgba);
                    }
                }
            }
        }
    }

    // ---------- 3D ----------

    /// Camera looking from `pos` at `target`, y up, vertical field of view in degrees.
    pub fn begin_3d(&mut self, pos: Vec3, target: Vec3, fovy_deg: f32) {
        self.cam = pos;
        let f = norm(sub(target, pos));
        let s = norm(cross(f, [0.0, 1.0, 0.0]));
        let u = cross(s, f);
        let view = [
            [s[0], s[1], s[2], -dot(s, pos)],
            [u[0], u[1], u[2], -dot(u, pos)],
            [-f[0], -f[1], -f[2], dot(f, pos)],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let t = 1.0 / (fovy_deg.to_radians() / 2.0).tan();
        let aspect = W as f32 / H as f32;
        let proj = [
            [t / aspect, 0.0, 0.0, 0.0],
            [0.0, t, 0.0, 0.0],
            [0.0, 0.0, (FAR + NEAR) / (NEAR - FAR), 2.0 * FAR * NEAR / (NEAR - FAR)],
            [0.0, 0.0, -1.0, 0.0],
        ];
        self.view_proj = mul(&proj, &view);
    }

    fn to_clip(&self, v: Vec3) -> Clip {
        let m = &self.view_proj;
        let mut out = [0.0; 4];
        for (i, o) in out.iter_mut().enumerate() {
            *o = m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2] + m[i][3];
        }
        out
    }

    /// True if the box is entirely outside one clip plane (so nothing in it is visible).
    fn box_hidden(&self, min: Vec3, max: Vec3) -> bool {
        let mut outside = [true; 6];
        for i in 0..8 {
            let p = [if i & 1 == 0 { min[0] } else { max[0] }, if i & 2 == 0 { min[1] } else { max[1] }, if i & 4 == 0 { min[2] } else { max[2] }];
            let c = self.to_clip(p);
            let tests = [c[0] > c[3], c[0] < -c[3], c[1] > c[3], c[1] < -c[3], c[2] > c[3], c[2] < -c[3]];
            for (o, t) in outside.iter_mut().zip(tests) {
                *o &= t;
            }
        }
        outside.iter().any(|&o| o)
    }

    /// Faces packed by the Lean mesher: 8 bytes each, x y z face r g b a. The box
    /// is the section's extent, for culling whole sections.
    pub fn section(&mut self, origin: [u32; 3], size: u32, faces: &[u8]) {
        let min = [origin[0] as f32, origin[1] as f32, origin[2] as f32];
        let max = [min[0] + size as f32, min[1] + size as f32, min[2] + size as f32];
        if faces.len() < 8 || self.box_hidden(min, max) {
            return;
        }
        for f in faces.chunks_exact(8) {
            let (x, y, z) = (f[0] as f32, f[1] as f32, f[2] as f32);
            let face = (f[3] as usize).min(5);
            let rgba = (f[4] as u32) << 24 | (f[5] as u32) << 16 | (f[6] as u32) << 8 | f[7] as u32;
            self.cube_face([x, y, z], [1.0, 1.0, 1.0], face, rgba);
        }
    }

    /// One face of an axis-aligned box with corner `at` and size `size`; back faces are skipped.
    fn cube_face(&mut self, at: Vec3, size: Vec3, face: usize, rgba: u32) {
        let (n, corners) = CUBE_FACES[face];
        let p = |c: [f32; 3]| [at[0] + c[0] * size[0], at[1] + c[1] * size[1], at[2] + c[2] * size[2]];
        let c0 = p(corners[0]);
        if dot(n, sub(self.cam, c0)) <= 0.0 {
            return;
        }
        let quad = [self.to_clip(c0), self.to_clip(p(corners[1])), self.to_clip(p(corners[2])), self.to_clip(p(corners[3]))];
        self.polygon(&quad, rgba);
    }

    /// A solid box centred at `center`.
    pub fn cube(&mut self, center: Vec3, size: Vec3, rgba: u32) {
        let at = [center[0] - size[0] / 2.0, center[1] - size[1] / 2.0, center[2] - size[2] / 2.0];
        if self.box_hidden(at, [at[0] + size[0], at[1] + size[1], at[2] + size[2]]) {
            return;
        }
        for face in 0..6 {
            self.cube_face(at, size, face, rgba);
        }
    }

    /// The twelve edges of a box centred at `center`.
    pub fn cube_wires(&mut self, center: Vec3, size: f32, rgba: u32) {
        let h = size / 2.0;
        let c = |i: usize| [center[0] + if i & 1 == 0 { -h } else { h }, center[1] + if i & 2 == 0 { -h } else { h }, center[2] + if i & 4 == 0 { -h } else { h }];
        for i in 0..8 {
            for bit in [1, 2, 4] {
                if i & bit == 0 {
                    self.line_3d(c(i), c(i | bit), rgba);
                }
            }
        }
    }

    /// A horizontal rectangle (the ocean) centred at `center`, `w` along x and `d` along z.
    pub fn plane(&mut self, center: Vec3, w: f32, d: f32, rgba: u32) {
        let (x0, x1) = (center[0] - w / 2.0, center[0] + w / 2.0);
        let (z0, z1) = (center[2] - d / 2.0, center[2] + d / 2.0);
        let y = center[1];
        let quad = [self.to_clip([x0, y, z0]), self.to_clip([x0, y, z1]), self.to_clip([x1, y, z1]), self.to_clip([x1, y, z0])];
        self.polygon(&quad, rgba);
    }

    fn line_3d(&mut self, a: Vec3, b: Vec3, rgba: u32) {
        let (mut ca, mut cb) = (self.to_clip(a), self.to_clip(b));
        // Clip at the near plane (z >= -w).
        let (da, db) = (ca[2] + ca[3], cb[2] + cb[3]);
        if da < 0.0 && db < 0.0 {
            return;
        }
        if da < 0.0 || db < 0.0 {
            let t = da / (da - db);
            let mid = [ca[0] + (cb[0] - ca[0]) * t, ca[1] + (cb[1] - ca[1]) * t, ca[2] + (cb[2] - ca[2]) * t, ca[3] + (cb[3] - ca[3]) * t];
            if da < 0.0 { ca = mid } else { cb = mid }
        }
        let sa = screen(ca);
        let sb = screen(cb);
        // Depth-tested, with a small bias so wires show on the faces they outline.
        let steps = (sa[0] - sb[0]).abs().max((sa[1] - sb[1]).abs()).ceil().max(1.0) as i32;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let (x, y) = ((sa[0] + (sb[0] - sa[0]) * t) as i32, (sa[1] + (sb[1] - sa[1]) * t) as i32);
            let z = sa[2] + (sb[2] - sa[2]) * t - 0.002;
            if (0..W as i32).contains(&x) && (0..H as i32).contains(&y) {
                let i = y as usize * W + x as usize;
                if z <= self.depth[i] {
                    self.color[i] = blend(self.color[i], rgba);
                }
            }
        }
    }

    /// A convex polygon (at most 4 vertices) in clip space, clipped at the near
    /// plane and drawn as a fan. Clipping a quad yields at most 5 vertices.
    fn polygon(&mut self, verts: &[Clip], rgba: u32) {
        let mut s = [[0.0f32; 3]; 6];
        let mut n = 0;
        for i in 0..verts.len() {
            let (a, b) = (verts[i], verts[(i + 1) % verts.len()]);
            let (da, db) = (a[2] + a[3], b[2] + b[3]);
            if da >= 0.0 {
                s[n] = screen(a);
                n += 1;
            }
            if (da >= 0.0) != (db >= 0.0) {
                let t = da / (da - db);
                s[n] = screen([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t, a[3] + (b[3] - a[3]) * t]);
                n += 1;
            }
        }
        if n < 3 {
            return;
        }
        for i in 1..n - 1 {
            self.triangle(s[0], s[i], s[i + 1], rgba);
        }
    }

    /// Screen-space triangle (x, y, depth) with a flat colour.
    fn triangle(&mut self, a: [f32; 3], b: [f32; 3], c: [f32; 3], rgba: u32) {
        let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if area.abs() < 1e-6 {
            return;
        }
        let (b, c) = if area < 0.0 { (c, b) } else { (b, c) };
        let area = area.abs();
        let min_x = a[0].min(b[0]).min(c[0]).floor().max(0.0) as i32;
        let max_x = a[0].max(b[0]).max(c[0]).ceil().min(W as f32 - 1.0) as i32;
        let min_y = a[1].min(b[1]).min(c[1]).floor().max(0.0) as i32;
        let max_y = a[1].max(b[1]).max(c[1]).ceil().min(H as f32 - 1.0) as i32;
        if min_x > max_x || min_y > max_y {
            return;
        }
        let edge = |p: [f32; 3], q: [f32; 3], x: f32, y: f32| (q[0] - p[0]) * (y - p[1]) - (q[1] - p[1]) * (x - p[0]);
        let write_depth = rgba & 255 >= 128;
        for y in min_y..=max_y {
            let py = y as f32 + 0.5;
            let row = y as usize * W;
            for x in min_x..=max_x {
                let px = x as f32 + 0.5;
                let (w0, w1, w2) = (edge(b, c, px, py), edge(c, a, px, py), edge(a, b, px, py));
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let z = (w0 * a[2] + w1 * b[2] + w2 * c[2]) / area;
                    let i = row + x as usize;
                    if z < self.depth[i] {
                        if write_depth {
                            self.depth[i] = z;
                        }
                        self.color[i] = blend(self.color[i], rgba);
                    }
                }
            }
        }
    }
}

/// Perspective divide to screen x, y and a depth value that grows with distance.
fn screen(c: Clip) -> [f32; 3] {
    let w = if c[3].abs() < 1e-6 { 1e-6 } else { c[3] };
    [(c[0] / w * 0.5 + 0.5) * W as f32, (0.5 - c[1] / w * 0.5) * H as f32, c[2] / w]
}

fn lerp_rgba(a: u32, b: u32, t: f32) -> u32 {
    let ch = |shift: u32| {
        let (x, y) = (((a >> shift) & 255) as f32, ((b >> shift) & 255) as f32);
        ((x + (y - x) * t).round().clamp(0.0, 255.0) as u32) << shift
    };
    ch(24) | ch(16) | ch(8) | ch(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_in_front_is_drawn_and_depth_tested() {
        let mut r = Renderer::new();
        r.begin_frame();
        r.begin_3d([0.5, 0.5, 5.0], [0.5, 0.5, 0.0], 60.0);
        r.cube([0.5, 0.5, 0.5], [1.0, 1.0, 1.0], 0xff0000ff);
        let centre = r.color[(H / 2) * W + W / 2];
        assert_eq!(centre, 0xff0000);
        // A farther cube behind it does not overwrite the centre pixel.
        r.cube([0.5, 0.5, -3.0], [1.0, 1.0, 1.0], 0x00ff00ff);
        assert_eq!(r.color[(H / 2) * W + W / 2], 0xff0000);
        // Nothing is drawn behind the camera.
        r.begin_frame();
        r.cube([0.5, 0.5, 9.0], [1.0, 1.0, 1.0], 0xff0000ff);
        assert!(r.color.iter().all(|&p| p == 0));
    }

    #[test]
    fn plane_under_camera_is_clipped_not_dropped() {
        let mut r = Renderer::new();
        r.begin_frame();
        r.begin_3d([48.0, 12.0, 48.0], [49.0, 11.7, 48.0], 61.0);
        r.plane([48.0, 10.5, 48.0], 108.0, 108.0, 0x2e7ea2ff);
        // The lower half of the screen shows the plane.
        assert_eq!(r.color[(H - 5) * W + W / 2], 0x2e7ea2);
        assert_eq!(r.color[2 * W + W / 2], 0);
    }

    #[test]
    fn section_faces_and_text() {
        let mut r = Renderer::new();
        r.begin_frame();
        r.begin_3d([2.5, 3.0, 6.0], [2.5, 1.0, 2.0], 61.0);
        let faces: Vec<u8> = [2u8, 1, 2, 0, 100, 150, 90, 255].to_vec();
        r.section([0, 0, 0], 16, &faces);
        assert!(r.color.iter().any(|&p| p == 0x64965a));
        r.text(0, 0, 1, 0xffffffff, "A");
        assert!(r.color[..8 * W].iter().any(|&p| p == 0xffffff));
    }
}
