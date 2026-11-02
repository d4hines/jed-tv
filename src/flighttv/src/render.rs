//! Software renderer for the city flight demo: a z-buffered flat-shaded
//! rasterizer at a low internal resolution, scaled up to the TV's 1280x720.
//! It draws the procedural city of `cityflight::world` (terrain, building
//! boxes), the aircraft, the filter's verified escape path, and a HUD.
//! World axes: x east, y north, z up (the demo's point-mass frame).

use cityflight::model::{State, GAM, PHI, PSI, X, Y, Z};
use cityflight::world::{World, MAP_HALF, TN, TSP};

pub const OUT_W: usize = 1280;
pub const OUT_H: usize = 720;
/// Internal resolution; each pixel becomes a 2x2 block on the TV.
pub const W: usize = 640;
pub const H: usize = 360;
const NEAR: f32 = 1.0;
const FAR: f32 = 2600.0;
/// Buildings farther than this from the camera are skipped.
const DRAW_DIST: f32 = 1500.0;
const SUN: [f32; 3] = [-0.45, -0.55, 0.70];
const FOG: [f32; 3] = [0.74, 0.81, 0.89];

#[derive(Clone, Copy)]
pub struct V3(pub f32, pub f32, pub f32);

impl V3 {
    fn sub(self, o: V3) -> V3 { V3(self.0 - o.0, self.1 - o.1, self.2 - o.2) }
    fn add(self, o: V3) -> V3 { V3(self.0 + o.0, self.1 + o.1, self.2 + o.2) }
    fn mul(self, k: f32) -> V3 { V3(self.0 * k, self.1 * k, self.2 * k) }
    fn dot(self, o: V3) -> f32 { self.0 * o.0 + self.1 * o.1 + self.2 * o.2 }
    fn cross(self, o: V3) -> V3 {
        V3(self.1 * o.2 - self.2 * o.1, self.2 * o.0 - self.0 * o.2, self.0 * o.1 - self.1 * o.0)
    }
    fn norm(self) -> V3 {
        let l = self.dot(self).sqrt();
        if l > 1e-9 { self.mul(1.0 / l) } else { self }
    }
}

/// Forward, left and up unit vectors of the aircraft body.
pub fn body_axes(s: &State) -> (V3, V3, V3) {
    let (sg, cg) = (s[GAM] as f32).sin_cos();
    let (sp, cp) = (s[PSI] as f32).sin_cos();
    let (sb, cb) = (s[PHI] as f32).sin_cos();
    let f = V3(cg * cp, cg * sp, sg);
    let l0 = V3(-sp, cp, 0.0);
    let u0 = f.cross(l0);
    (f, l0.mul(cb).sub(u0.mul(sb)), u0.mul(cb).add(l0.mul(sb)))
}

pub fn pos(s: &State) -> V3 {
    V3(s[X] as f32, s[Y] as f32, s[Z] as f32)
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub eye: V3,
    pub fwd: V3,
    pub right: V3,
    pub up: V3,
    /// Pixels per unit of tan(angle).
    pub focal: f32,
}

impl Camera {
    pub fn look(eye: V3, at: V3, up_hint: V3, fov_deg: f32) -> Camera {
        let fwd = at.sub(eye).norm();
        let right = fwd.cross(up_hint).norm();
        let up = right.cross(fwd);
        Camera { eye, fwd, right, up, focal: (W as f32 / 2.0) / (fov_deg.to_radians() / 2.0).tan() }
    }

    /// Camera space: (x right, y up, z forward).
    fn view(&self, p: V3) -> V3 {
        let d = p.sub(self.eye);
        V3(d.dot(self.right), d.dot(self.up), d.dot(self.fwd))
    }
}

pub struct Raster {
    pub color: Vec<u32>,
    depth: Vec<f32>,
    pub tris: usize,
}

fn pack(r: f32, g: f32, b: f32) -> u32 {
    let c = |v: f32| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
    c(r) << 16 | c(g) << 8 | c(b)
}

/// Mix a lit colour toward the fog by distance.
fn shade(base: [f32; 3], normal: V3, dist: f32) -> u32 {
    let sun = V3(SUN[0], SUN[1], SUN[2]).norm();
    let l = 0.45 + 0.55 * normal.dot(sun).max(0.0);
    let f = 1.0 - (-dist / 1400.0).exp();
    let mix = |c: f32, fog: f32| c * l * (1.0 - f) + fog * f;
    pack(mix(base[0], FOG[0]), mix(base[1], FOG[1]), mix(base[2], FOG[2]))
}

impl Raster {
    pub fn new() -> Raster {
        Raster { color: vec![0; W * H], depth: vec![0.0; W * H], tris: 0 }
    }

    /// Sky gradient; the depth buffer is reset to "infinitely far".
    fn clear(&mut self) {
        for y in 0..H {
            let t = y as f32 / H as f32;
            let c = pack(0.36 + (FOG[0] - 0.36) * t, 0.56 + (FOG[1] - 0.56) * t, 0.86 + (FOG[2] - 0.86) * t);
            self.color[y * W..(y + 1) * W].fill(c);
        }
        self.depth.fill(0.0);
        self.tris = 0;
    }

    /// A world-space triangle, clipped to the near plane, in colour `c`.
    fn tri(&mut self, cam: &Camera, a: V3, b: V3, cc: V3, c: u32) {
        let v = [cam.view(a), cam.view(b), cam.view(cc)];
        if v.iter().all(|p| p.2 < NEAR) || v.iter().all(|p| p.2 > FAR) {
            return;
        }
        // Clip against z = NEAR (Sutherland-Hodgman): up to four vertices.
        let mut poly: [V3; 4] = [V3(0.0, 0.0, 0.0); 4];
        let mut n = 0;
        for i in 0..3 {
            let (p, q) = (v[i], v[(i + 1) % 3]);
            let (pin, qin) = (p.2 >= NEAR, q.2 >= NEAR);
            if pin {
                poly[n] = p;
                n += 1;
            }
            if pin != qin {
                let t = (NEAR - p.2) / (q.2 - p.2);
                poly[n] = p.add(q.sub(p).mul(t));
                n += 1;
            }
        }
        if n < 3 {
            return;
        }
        let proj = |p: V3| {
            let iz = 1.0 / p.2;
            (W as f32 / 2.0 + cam.focal * p.0 * iz, H as f32 / 2.0 - cam.focal * p.1 * iz, iz)
        };
        let s: Vec<(f32, f32, f32)> = poly[..n].iter().map(|&p| proj(p)).collect();
        for i in 1..n - 1 {
            self.fill(s[0], s[i], s[i + 1], c);
        }
    }

    /// Screen-space triangle with 1/z depth test.
    fn fill(&mut self, a: (f32, f32, f32), b: (f32, f32, f32), c: (f32, f32, f32), colour: u32) {
        let area = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
        if area.abs() < 1e-6 {
            return;
        }
        let (x0, x1) = (a.0.min(b.0).min(c.0).floor().max(0.0) as i32, a.0.max(b.0).max(c.0).ceil().min(W as f32 - 1.0) as i32);
        let (y0, y1) = (a.1.min(b.1).min(c.1).floor().max(0.0) as i32, a.1.max(b.1).max(c.1).ceil().min(H as f32 - 1.0) as i32);
        if x0 > x1 || y0 > y1 {
            return;
        }
        self.tris += 1;
        let inv = 1.0 / area;
        for y in y0..=y1 {
            let py = y as f32 + 0.5;
            for x in x0..=x1 {
                let px = x as f32 + 0.5;
                let w0 = ((b.0 - px) * (c.1 - py) - (b.1 - py) * (c.0 - px)) * inv;
                let w1 = ((c.0 - px) * (a.1 - py) - (c.1 - py) * (a.0 - px)) * inv;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let iz = w0 * a.2 + w1 * b.2 + w2 * c.2;
                let i = y as usize * W + x as usize;
                if iz > self.depth[i] {
                    self.depth[i] = iz;
                    self.color[i] = colour;
                }
            }
        }
    }

    /// A quad (a, b, c, d in order) lit by its normal.
    fn quad(&mut self, cam: &Camera, p: [V3; 4], base: [f32; 3]) {
        let n = p[1].sub(p[0]).cross(p[3].sub(p[0])).norm();
        let centre = p[0].add(p[2]).mul(0.5);
        // Back-face culling keeps the box interiors out.
        if n.dot(centre.sub(cam.eye)) > 0.0 {
            return;
        }
        let c = shade(base, n, centre.sub(cam.eye).dot(cam.fwd).max(0.0));
        self.tri(cam, p[0], p[1], p[2], c);
        self.tri(cam, p[0], p[2], p[3], c);
    }

    /// An axis-aligned box: top and four walls (no floor).
    fn boxed(&mut self, cam: &Camera, lo: V3, hi: V3, base: [f32; 3]) {
        let (x0, y0, z0, x1, y1, z1) = (lo.0, lo.1, lo.2, hi.0, hi.1, hi.2);
        let top = [0.95, 0.95, 0.95].map(|k| base[2] * k);
        self.quad(cam, [V3(x0, y0, z1), V3(x1, y0, z1), V3(x1, y1, z1), V3(x0, y1, z1)], [base[0] * 1.05, base[1] * 1.05, top[0]]);
        self.quad(cam, [V3(x0, y0, z0), V3(x0, y0, z1), V3(x1, y0, z1), V3(x1, y0, z0)], base);
        self.quad(cam, [V3(x1, y1, z0), V3(x1, y1, z1), V3(x0, y1, z1), V3(x0, y1, z0)], base);
        self.quad(cam, [V3(x1, y0, z0), V3(x1, y0, z1), V3(x1, y1, z1), V3(x1, y1, z0)], base);
        self.quad(cam, [V3(x0, y1, z0), V3(x0, y1, z1), V3(x0, y0, z1), V3(x0, y0, z0)], base);
    }

    /// Screen-space line (no depth test), for paths over the scene.
    fn line(&mut self, cam: &Camera, a: V3, b: V3, c: u32) {
        let (mut p, mut q) = (cam.view(a), cam.view(b));
        if p.2 < NEAR && q.2 < NEAR {
            return;
        }
        if p.2 < NEAR {
            p = p.add(q.sub(p).mul((NEAR - p.2) / (q.2 - p.2)));
        }
        if q.2 < NEAR {
            q = q.add(p.sub(q).mul((NEAR - q.2) / (p.2 - q.2)));
        }
        let s = |v: V3| (W as f32 / 2.0 + cam.focal * v.0 / v.2, H as f32 / 2.0 - cam.focal * v.1 / v.2);
        let ((x0, y0), (x1, y1)) = (s(p), s(q));
        let steps = (x1 - x0).abs().max((y1 - y0).abs()).ceil().clamp(1.0, 2000.0) as usize;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let (x, y) = ((x0 + (x1 - x0) * t) as i32, (y0 + (y1 - y0) * t) as i32);
            if (0..W as i32).contains(&x) && (0..H as i32).contains(&y) {
                let i = y as usize * W + x as usize;
                self.color[i] = c;
                if x + 1 < W as i32 {
                    self.color[i + 1] = c;
                }
            }
        }
    }

    fn terrain(&mut self, cam: &Camera, world: &World) {
        // 40 m quads within DRAW_DIST of the camera, finer nearby is not needed at this resolution.
        let step = 2usize;
        let cells = (DRAW_DIST / (TSP as f32 * step as f32)) as i32 + 1;
        let (ci, cj) = (((cam.eye.0 + MAP_HALF as f32) / TSP as f32) as i32, ((cam.eye.1 + MAP_HALF as f32) / TSP as f32) as i32);
        let at = |i: i32, j: i32| {
            let (i, j) = (i.clamp(0, TN as i32 - 1) as usize, j.clamp(0, TN as i32 - 1) as usize);
            V3(-MAP_HALF as f32 + i as f32 * TSP as f32, -MAP_HALF as f32 + j as f32 * TSP as f32, world.terr[j * TN + i])
        };
        let s = step as i32;
        let mut j = (cj / s - cells) * s;
        while j < (cj / s + cells) * s {
            let mut i = (ci / s - cells) * s;
            while i < (ci / s + cells) * s {
                if (0..TN as i32).contains(&i) && (0..TN as i32).contains(&j) {
                    let p = [at(i, j), at(i + s, j), at(i + s, j + s), at(i, j + s)];
                    let c = p[0].add(p[2]).mul(0.5);
                    if c.sub(cam.eye).dot(cam.fwd) > -60.0 {
                        let h = c.2;
                        let park = world.is_park(c.0 as f64, c.1 as f64);
                        let base = if park { [0.30, 0.52, 0.26] } else if h > 300.0 { [0.52, 0.50, 0.46] } else if h > 90.0 { [0.42, 0.50, 0.32] } else { [0.36, 0.40, 0.38] };
                        let n = p[1].sub(p[0]).cross(p[3].sub(p[0])).norm();
                        let col = shade(base, n, c.sub(cam.eye).dot(cam.fwd).max(0.0));
                        self.tri(cam, p[0], p[1], p[2], col);
                        self.tri(cam, p[0], p[2], p[3], col);
                    }
                }
                i += s;
            }
            j += s;
        }
    }

    fn buildings(&mut self, cam: &Camera, world: &World) {
        for b in &world.boxes {
            let lo = V3(b.x0 as f32, b.y0 as f32, b.z0 as f32);
            let hi = V3(b.x1 as f32, b.y1 as f32, b.z1 as f32);
            let c = lo.add(hi).mul(0.5);
            let d = c.sub(cam.eye);
            let dist = d.dot(d).sqrt();
            if dist > DRAW_DIST || d.dot(cam.fwd) < -(hi.sub(lo).dot(hi.sub(lo)).sqrt()) {
                continue;
            }
            // Tiny far boxes cost triangles and show nothing at this resolution.
            if (hi.2 - lo.2) * cam.focal / dist < 1.5 && (hi.0 - lo.0).max(hi.1 - lo.1) * cam.focal / dist < 2.0 {
                continue;
            }
            let t = b.tint;
            let base = [0.62 + 0.18 * t, 0.60 + 0.12 * t, 0.58 + 0.06 * (1.0 - t)];
            self.boxed(cam, lo, hi, base);
        }
    }

    /// A simple Cessna: fuselage, wing, tailplane and fin from body axes.
    fn aircraft(&mut self, cam: &Camera, s: &State) {
        let p = pos(s);
        let (f, l, u) = body_axes(s);
        let at = |x: f32, y: f32, z: f32| p.add(f.mul(x)).add(l.mul(y)).add(u.mul(z));
        let white = [0.92, 0.92, 0.95];
        let red = [0.85, 0.15, 0.12];
        let n = |a: V3, b: V3, c: V3| b.sub(a).cross(c.sub(a)).norm();
        let mut tri = |a: V3, b: V3, c: V3, base: [f32; 3]| {
            let nn = n(a, b, c);
            let col = shade(base, nn, a.sub(cam.eye).dot(cam.fwd).max(0.0));
            self.tri(cam, a, b, c, col);
            self.tri(cam, a, c, b, col);
        };
        // Wing (high wing): a thin slab from -5.5 to 5.5 m, chord 1.5 m.
        tri(at(0.6, -5.5, 1.0), at(0.6, 5.5, 1.0), at(-0.9, 5.5, 1.0), white);
        tri(at(0.6, -5.5, 1.0), at(-0.9, 5.5, 1.0), at(-0.9, -5.5, 1.0), white);
        // Fuselage: a long triangle prism (two sides and a top).
        tri(at(3.5, 0.0, 0.0), at(-3.5, -0.6, 0.6), at(-3.5, -0.6, -0.5), white);
        tri(at(3.5, 0.0, 0.0), at(-3.5, 0.6, -0.5), at(-3.5, 0.6, 0.6), white);
        tri(at(3.5, 0.0, 0.0), at(-3.5, 0.6, 0.6), at(-3.5, -0.6, 0.6), white);
        tri(at(3.5, 0.0, 0.0), at(-3.5, -0.6, -0.5), at(-3.5, 0.6, -0.5), white);
        // Tailplane and fin.
        tri(at(-3.0, -1.7, 0.2), at(-3.0, 1.7, 0.2), at(-3.8, 0.0, 0.2), red);
        tri(at(-2.8, 0.0, 0.3), at(-3.8, 0.0, 0.3), at(-3.8, 0.0, 1.6), red);
    }

    /// The whole frame. `path` is the filter's verified escape (world points).
    pub fn frame(&mut self, world: &World, s: &State, cam_mode: u8, path: &[[f64; 3]], path_ok: bool) -> Camera {
        self.clear();
        let p = pos(s);
        let (f, _, u) = body_axes(s);
        let cam = match cam_mode {
            1 => Camera::look(p.add(f.mul(2.0)).add(u.mul(1.2)), p.add(f.mul(60.0)).add(u.mul(1.2)), u, 70.0),
            _ => {
                let eye = p.sub(f.mul(28.0)).add(V3(0.0, 0.0, 9.0));
                Camera::look(eye, p.add(f.mul(25.0)), V3(0.0, 0.0, 1.0), 62.0)
            }
        };
        self.terrain(&cam, world);
        self.buildings(&cam, world);
        if cam_mode != 1 {
            self.aircraft(&cam, s);
        }
        let c = if path_ok { pack(0.2, 1.0, 0.3) } else { pack(1.0, 0.3, 0.2) };
        for w in path.windows(2) {
            self.line(&cam, V3(w[0][0] as f32, w[0][1] as f32, w[0][2] as f32), V3(w[1][0] as f32, w[1][1] as f32, w[1][2] as f32), c);
        }
        cam
    }

    /// Scale the internal frame 2x into a 1280x720 0x00RRGGBB buffer.
    pub fn upscale(&self, out: &mut [u32]) {
        for y in 0..H {
            let src = &self.color[y * W..(y + 1) * W];
            let row = &mut out[2 * y * OUT_W..(2 * y + 1) * OUT_W];
            for (x, &c) in src.iter().enumerate() {
                row[2 * x] = c;
                row[2 * x + 1] = c;
            }
            let (a, b) = out.split_at_mut((2 * y + 1) * OUT_W);
            b[..OUT_W].copy_from_slice(&a[2 * y * OUT_W..]);
        }
    }
}

impl Default for Raster {
    fn default() -> Self {
        Self::new()
    }
}

/// 8x8 bitmap text at integer scale onto a 1280x720 buffer; returns the width drawn.
pub fn text(out: &mut [u32], x: i32, y: i32, scale: i32, colour: u32, s: &str) -> i32 {
    use font8x8::legacy::BASIC_LEGACY;
    for (i, ch) in s.chars().enumerate() {
        let glyph = BASIC_LEGACY.get(ch as usize).copied().unwrap_or([0; 8]);
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if bits & (1 << col) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let (px, py) = (x + (i as i32 * 8 + col) * scale + dx, y + row as i32 * scale + dy);
                            if (0..OUT_W as i32).contains(&px) && (0..OUT_H as i32).contains(&py) {
                                out[py as usize * OUT_W + px as usize] = colour;
                            }
                        }
                    }
                }
            }
        }
    }
    s.chars().count() as i32 * 8 * scale
}

/// A translucent dark box behind HUD text.
pub fn panel(out: &mut [u32], x: i32, y: i32, w: i32, h: i32) {
    for py in y.max(0)..(y + h).min(OUT_H as i32) {
        for px in x.max(0)..(x + w).min(OUT_W as i32) {
            let p = &mut out[py as usize * OUT_W + px as usize];
            *p = (*p >> 2) & 0x3f3f3f;
        }
    }
}
