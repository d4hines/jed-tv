//! flightgpu: the city flight demo's picture, drawn by the TV's Mali GPU.
//!
//! Runs in the native PID 1 session as an Android-ABI process under the stock
//! linker (src/pid1/scripts/flight-gpu.sh). It builds the same
//! procedural city as flighttv (World::new(1337) is deterministic), announces
//! a shared state record (flighttv::state), and whenever flighttv publishes a
//! new aircraft state it draws the terrain, the buildings, the aircraft and
//! the filter's verified escape with GLES 2 into an off-screen buffer, reads
//! the pixels back, overlays the same HUD and publishes the frame through a
//! second shared file, which flighttv presents on the display (the display
//! service owns the panel in the native session; /dev/fb0 is not it). The
//! simulation and the Lean flight model stay in flighttv.
//!
//! Arguments: [state file]   (default /run/flight-gpu/state)

mod gles;

use cityflight::model::State;
use cityflight::world::{World, MAP_HALF, TN, TSP};
use flighttv::render::{body_axes, pos, text, V3, OUT_H, OUT_W};
use flighttv::state::{self, Frame, Published, Shared, READY_FILE};
use flighttv::{hud, Overlay};
use gles::*;
use std::ffi::c_void;
use std::io::Write;
use std::time::{Duration, Instant};

const W: i32 = OUT_W as i32;
const H: i32 = OUT_H as i32;
const SUN: [f32; 3] = [-0.45, -0.55, 0.70];
const FOG: [f32; 3] = [0.74, 0.81, 0.89];

const VS: &str = "attribute vec3 p; attribute vec3 c; uniform mat4 mvp; uniform vec3 eye; varying vec3 v; varying float d;
void main() { v = c; d = length(p - eye); gl_Position = mvp * vec4(p, 1.0); }";
const FS: &str = "precision mediump float; varying vec3 v; varying float d; uniform vec3 fog; uniform float fogScale;
void main() { float f = 1.0 - exp(-d / fogScale); gl_FragColor = vec4(mix(v, fog, f), 1.0); }";

fn log(msg: impl AsRef<str>) {
    println!("flightgpu: {}", msg.as_ref());
}

// --- small vector helpers on [f32; 3] (the renderer's V3 keeps its operations private) ---
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn mul(a: [f32; 3], k: f32) -> [f32; 3] { [a[0] * k, a[1] * k, a[2] * k] }
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn norm(a: [f32; 3]) -> [f32; 3] { let l = dot(a, a).sqrt(); if l > 1e-9 { mul(a, 1.0 / l) } else { a } }
fn v(a: V3) -> [f32; 3] { [a.0, a.1, a.2] }

/// Lit colour of a face: the same rule as the CPU renderer, before fog (the shader adds fog).
fn lit(base: [f32; 3], normal: [f32; 3]) -> [f32; 3] {
    let l = 0.45 + 0.55 * dot(normal, norm(SUN)).max(0.0);
    [base[0] * l, base[1] * l, base[2] * l]
}

/// Push a quad (a, b, c, d in order) as two triangles with one lit colour.
fn quad(out: &mut Vec<f32>, p: [[f32; 3]; 4], base: [f32; 3]) {
    let n = norm(cross(sub(p[1], p[0]), sub(p[3], p[0])));
    let c = lit(base, n);
    for i in [0, 1, 2, 0, 2, 3] {
        out.extend_from_slice(&p[i]);
        out.extend_from_slice(&c);
    }
}

/// The city: every building as five faces, and the terrain as 40 m quads. Built once.
fn city(world: &World) -> (Vec<f32>, Vec<f32>) {
    let mut boxes = Vec::with_capacity(world.boxes.len() * 30 * 6);
    for b in &world.boxes {
        let (x0, y0, z0, x1, y1, z1) = (b.x0 as f32, b.y0 as f32, b.z0 as f32, b.x1 as f32, b.y1 as f32, b.z1 as f32);
        let t = b.tint;
        let base = [0.62 + 0.18 * t, 0.60 + 0.12 * t, 0.58 + 0.06 * (1.0 - t)];
        let top = [base[0] * 1.05, base[1] * 1.05, base[2] * 0.95];
        quad(&mut boxes, [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]], top);
        quad(&mut boxes, [[x0, y0, z0], [x0, y0, z1], [x1, y0, z1], [x1, y0, z0]], base);
        quad(&mut boxes, [[x1, y1, z0], [x1, y1, z1], [x0, y1, z1], [x0, y1, z0]], base);
        quad(&mut boxes, [[x1, y0, z0], [x1, y0, z1], [x1, y1, z1], [x1, y0, z0]], base);
        quad(&mut boxes, [[x0, y1, z0], [x0, y1, z1], [x0, y0, z1], [x0, y0, z0]], base);
    }
    let step = 2usize;
    let at = |i: usize, j: usize| {
        let (i, j) = (i.min(TN - 1), j.min(TN - 1));
        [-MAP_HALF as f32 + i as f32 * TSP as f32, -MAP_HALF as f32 + j as f32 * TSP as f32, world.terr[j * TN + i]]
    };
    let mut terrain = Vec::with_capacity((TN / step) * (TN / step) * 36);
    let mut j = 0;
    while j + step < TN {
        let mut i = 0;
        while i + step < TN {
            let p = [at(i, j), at(i + step, j), at(i + step, j + step), at(i, j + step)];
            let c = mul(add(p[0], p[2]), 0.5);
            let h = c[2];
            let park = world.is_park(c[0] as f64, c[1] as f64);
            let base = if park { [0.30, 0.52, 0.26] } else if h > 300.0 { [0.52, 0.50, 0.46] } else if h > 90.0 { [0.42, 0.50, 0.32] } else { [0.36, 0.40, 0.38] };
            quad(&mut terrain, p, base);
            i += step;
        }
        j += step;
    }
    (boxes, terrain)
}

/// The aircraft, as the CPU renderer draws it, both-sided.
fn aircraft(out: &mut Vec<f32>, s: &State) {
    let p = v(pos(s));
    let (f, l, u) = body_axes(s);
    let (f, l, u) = (v(f), v(l), v(u));
    let at = |x: f32, y: f32, z: f32| add(add(add(p, mul(f, x)), mul(l, y)), mul(u, z));
    let white = [0.92, 0.92, 0.95];
    let red = [0.85, 0.15, 0.12];
    let mut tri = |a: [f32; 3], b: [f32; 3], c: [f32; 3], base: [f32; 3]| {
        let col = lit(base, norm(cross(sub(b, a), sub(c, a))));
        for q in [a, b, c, a, c, b] {
            out.extend_from_slice(&q);
            out.extend_from_slice(&col);
        }
    };
    tri(at(0.6, -5.5, 1.0), at(0.6, 5.5, 1.0), at(-0.9, 5.5, 1.0), white);
    tri(at(0.6, -5.5, 1.0), at(-0.9, 5.5, 1.0), at(-0.9, -5.5, 1.0), white);
    tri(at(3.5, 0.0, 0.0), at(-3.5, -0.6, 0.6), at(-3.5, -0.6, -0.5), white);
    tri(at(3.5, 0.0, 0.0), at(-3.5, 0.6, -0.5), at(-3.5, 0.6, 0.6), white);
    tri(at(3.5, 0.0, 0.0), at(-3.5, 0.6, 0.6), at(-3.5, -0.6, 0.6), white);
    tri(at(3.5, 0.0, 0.0), at(-3.5, -0.6, -0.5), at(-3.5, 0.6, -0.5), white);
    tri(at(-3.0, -1.7, 0.2), at(-3.0, 1.7, 0.2), at(-3.8, 0.0, 0.2), red);
    tri(at(-2.8, 0.0, 0.3), at(-3.8, 0.0, 0.3), at(-3.8, 0.0, 1.6), red);
}

/// Column-major perspective * view for the chase or cockpit camera, like the CPU renderer's.
fn camera(s: &State, cam_mode: u8) -> ([f32; 16], [f32; 3]) {
    let p = v(pos(s));
    let (f, _, u) = body_axes(s);
    let (f, u) = (v(f), v(u));
    let (eye, at, up_hint, fov) = match cam_mode {
        1 => (add(add(p, mul(f, 2.0)), mul(u, 1.2)), add(add(p, mul(f, 60.0)), mul(u, 1.2)), u, 70.0f32),
        _ => (add(sub(p, mul(f, 28.0)), [0.0, 0.0, 9.0]), add(p, mul(f, 25.0)), [0.0, 0.0, 1.0], 62.0f32),
    };
    let fwd = norm(sub(at, eye));
    let s_ = norm(cross(fwd, up_hint));
    let up = cross(s_, fwd);
    let view = [s_[0], up[0], -fwd[0], 0.0, s_[1], up[1], -fwd[1], 0.0, s_[2], up[2], -fwd[2], 0.0, -dot(s_, eye), -dot(up, eye), dot(fwd, eye), 1.0];
    let (near, far) = (1.0f32, 2600.0f32);
    let t = 1.0 / (fov.to_radians() / 2.0).tan();
    let aspect = W as f32 / H as f32;
    let proj = [t / aspect, 0.0, 0.0, 0.0, 0.0, t, 0.0, 0.0, 0.0, 0.0, (far + near) / (near - far), -1.0, 0.0, 0.0, 2.0 * far * near / (near - far), 0.0];
    let mut m = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            m[col * 4 + row] = (0..4).map(|k| proj[k * 4 + row] * view[col * 4 + k]).sum();
        }
    }
    (m, eye)
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "/run/flight-gpu/state".into());
    let mut frame = vec![0u32; OUT_W * OUT_H];
    let t0 = Instant::now();
    let ctx = Context::new(W, H);
    log(format!("GLES on {} / {} in {:.2} s", ctx.vendor, ctx.renderer, t0.elapsed().as_secs_f64()));
    let gl = &ctx.gl;
    let program = ctx.program(VS, FS);

    let t0 = Instant::now();
    let world: &'static World = Box::leak(Box::new(World::new(1337)));
    let (boxes, terrain) = city(world);
    log(format!("city built in {:.1} s: {} buildings, {} vertices", t0.elapsed().as_secs_f64(), world.boxes.len(), (boxes.len() + terrain.len()) / 6));
    let (p_attr, c_attr, u_mvp, u_eye, u_fog, u_scale, vbo) = unsafe {
        (gl.UseProgram)(program);
        let mut vbo = [0u32; 3];
        (gl.GenBuffers)(3, vbo.as_mut_ptr());
        for (buffer, data) in [(vbo[0], &boxes), (vbo[1], &terrain)] {
            (gl.BindBuffer)(GL_ARRAY_BUFFER, buffer);
            (gl.BufferData)(GL_ARRAY_BUFFER, (data.len() * 4) as isize, data.as_ptr() as *const c_void, GL_STATIC_DRAW);
        }
        (gl.Enable)(GL_DEPTH_TEST);
        (gl.Enable)(GL_CULL_FACE);
        (gl.Viewport)(0, 0, W, H);
        (gl.ClearColor)(0.36, 0.56, 0.86, 1.0);
        ((gl.GetAttribLocation)(program, c"p".as_ptr()) as u32, (gl.GetAttribLocation)(program, c"c".as_ptr()) as u32,
         (gl.GetUniformLocation)(program, c"mvp".as_ptr()), (gl.GetUniformLocation)(program, c"eye".as_ptr()),
         (gl.GetUniformLocation)(program, c"fog".as_ptr()), (gl.GetUniformLocation)(program, c"fogScale".as_ptr()), vbo)
    };
    let draw = |buffer: u32, mode: u32, first: i32, count: i32| unsafe {
        (gl.BindBuffer)(GL_ARRAY_BUFFER, buffer);
        (gl.VertexAttribPointer)(p_attr, 3, GL_FLOAT, 0, 24, std::ptr::null());
        (gl.VertexAttribPointer)(c_attr, 3, GL_FLOAT, 0, 24, 12 as *const c_void);
        (gl.EnableVertexAttribArray)(p_attr);
        (gl.EnableVertexAttribArray)(c_attr);
        (gl.DrawArrays)(mode, first, count);
    };

    // Announce the shared record and the frame; flighttv maps both when it starts.
    let _ = std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap_or(std::path::Path::new("/")));
    let (shared, gpu_frame): (&'static mut Shared, &'static mut Frame) = match (state::open(&path, true), state::open_frame(&path, true)) {
        (Ok(s), Ok(f)) => (s, f),
        (Err(e), _) | (_, Err(e)) => {
            log(format!("state {path}: {e}"));
            std::process::exit(1);
        }
    };
    for file in [path.clone(), state::frame_path(&path)] {
        let _ = std::fs::set_permissions(&file, std::os::unix::fs::PermissionsExt::from_mode(0o666));
    }
    std::fs::File::create(READY_FILE).and_then(|mut f| writeln!(f, "{path}")).expect("ready file");
    log(format!("ready: state {path}; start City flight on the TV"));

    let mut rgba = vec![0u8; (W * H * 4) as usize];
    let mut dynamic: Vec<f32> = Vec::new();
    let mut last_seq = 0;
    let mut frames = 0u32;
    let mut window = Instant::now();
    let mut gpu_ms = 0.0;
    loop {
        let Some((seq, p)) = shared.read() else {
            std::thread::sleep(Duration::from_millis(2));
            continue;
        };
        if seq == last_seq {
            std::thread::sleep(Duration::from_millis(4));
            continue;
        }
        last_seq = seq;
        let Published { snap, fps: _, speed, show_help, cam_mode, status } = p;
        let t0 = Instant::now();
        let (mvp, eye) = camera(&snap.s, cam_mode);
        dynamic.clear();
        if cam_mode != 1 {
            aircraft(&mut dynamic, &snap.s);
        }
        let aircraft_verts = (dynamic.len() / 6) as i32;
        let path_colour = if snap.accepted { [0.2, 1.0, 0.3] } else { [1.0, 0.3, 0.2] };
        for w in snap.path.windows(2) {
            for q in w {
                dynamic.extend_from_slice(&[q[0] as f32, q[1] as f32, q[2] as f32]);
                dynamic.extend_from_slice(&path_colour);
            }
        }
        unsafe {
            (gl.UniformMatrix4fv)(u_mvp, 1, 0, mvp.as_ptr());
            (gl.Uniform3f)(u_eye, eye[0], eye[1], eye[2]);
            (gl.Uniform3f)(u_fog, FOG[0], FOG[1], FOG[2]);
            (gl.Uniform1f)(u_scale, 1400.0);
            (gl.Clear)(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            draw(vbo[1], GL_TRIANGLES, 0, (terrain.len() / 6) as i32);
            draw(vbo[0], GL_TRIANGLES, 0, (boxes.len() / 6) as i32);
            (gl.BindBuffer)(GL_ARRAY_BUFFER, vbo[2]);
            (gl.BufferData)(GL_ARRAY_BUFFER, (dynamic.len() * 4) as isize, dynamic.as_ptr() as *const c_void, GL_DYNAMIC_DRAW);
            (gl.Disable)(GL_CULL_FACE);
            draw(vbo[2], GL_TRIANGLES, 0, aircraft_verts);
            (gl.Enable)(GL_CULL_FACE);
            (gl.Uniform1f)(u_scale, 1e9); // the path is drawn without fog
            (gl.LineWidth)(3.0);
            draw(vbo[2], GL_LINES, aircraft_verts, (dynamic.len() / 6) as i32 - aircraft_verts);
            (gl.Finish)();
            (gl.ReadPixels)(0, 0, W, H, GL_RGBA, GL_UNSIGNED_BYTE, rgba.as_mut_ptr() as *mut c_void);
        }
        // GL rows are bottom-up; the HUD and the display want top-down 0x00RRGGBB.
        for y in 0..OUT_H {
            let src = &rgba[(OUT_H - 1 - y) * OUT_W * 4..(OUT_H - y) * OUT_W * 4];
            for (d, px) in frame[y * OUT_W..(y + 1) * OUT_W].iter_mut().zip(src.chunks_exact(4)) {
                *d = (px[0] as u32) << 16 | (px[1] as u32) << 8 | px[2] as u32;
            }
        }
        gpu_ms = 0.8 * gpu_ms + 0.2 * t0.elapsed().as_secs_f64() * 1e3;
        let fps = frames as f64 / window.elapsed().as_secs_f64().max(1e-3);
        hud(&mut frame, &snap, &Overlay { fps, speed, show_help, status: &status });
        text(&mut frame, OUT_W as i32 - 330, OUT_H as i32 - 20, 2, 0xb0b4c0, &format!("GPU frame {gpu_ms:.1} ms"));
        gpu_frame.publish(&frame);
        frames += 1;
        if window.elapsed() >= Duration::from_secs(2) {
            log(format!("{fps:.1} fps, GPU frame {gpu_ms:.1} ms, glGetError=0x{:x}", unsafe { (gl.GetError)() }));
            frames = 0;
            window = Instant::now();
        }
    }
}
