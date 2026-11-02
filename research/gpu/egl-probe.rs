//! GPU probe for the native PID 1 session on the G06: draws a city of boxes
//! with OpenGL ES 2 on the Mali-G31 through the stock Android GL stack, off
//! screen, reads the pixels back and shows them on /dev/fb0.
//!
//! Android ABI (armv7-linux-androideabi), run under the stock Android linker
//! with the system and vendor libraries mounted (mount-stock-runtime.sh) and
//! the stock mali_kbase module loaded. Only dlopen is linked; libEGL.so and
//! libGLESv2.so are Android's loaders, which pick /vendor/lib/egl/libGLES_mali.so.
//! No window system is involved: the surface is an EGL pbuffer.
//!
//! Arguments: [boxes] [frames]   (default 2000 boxes, 60 frames)
//! Prints GPU_PROBE lines with the driver strings, the time per frame and the
//! time to read a frame back, then GPU_PROBE_PASS. Writes /tmp/gpu-frame.rgba.

use std::ffi::{c_char, c_void, CStr, CString};
use std::io::Write;
use std::os::fd::AsRawFd;
use std::time::Instant;

extern "C" {
    fn dlopen(name: *const c_char, flags: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn ioctl(fd: i32, request: u32, ...) -> i32;
    fn mmap64(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut c_void;
}

const W: i32 = 1280;
const H: i32 = 720;

struct Lib(*mut c_void);

impl Lib {
    fn open(name: &str) -> Lib {
        let c = CString::new(name).unwrap();
        let h = unsafe { dlopen(c.as_ptr(), 2) };
        if h.is_null() {
            let e = unsafe { dlerror() };
            panic!("dlopen {name}: {}", if e.is_null() { "unknown".into() } else { unsafe { CStr::from_ptr(e) }.to_string_lossy() });
        }
        Lib(h)
    }
    fn sym<T: Copy>(&self, name: &str) -> T {
        let c = CString::new(name).unwrap();
        let p = unsafe { dlsym(self.0, c.as_ptr()) };
        assert!(!p.is_null(), "missing symbol {name}");
        assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<*mut c_void>());
        unsafe { std::mem::transmute_copy(&p) }
    }
}

type EglDisplay = *mut c_void;
type EglConfig = *mut c_void;
type EglSurface = *mut c_void;
type EglContext = *mut c_void;

const EGL_NONE: i32 = 0x3038;
const EGL_SURFACE_TYPE: i32 = 0x3033;
const EGL_PBUFFER_BIT: i32 = 0x0001;
const EGL_RENDERABLE_TYPE: i32 = 0x3040;
const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
const EGL_RED_SIZE: i32 = 0x3024;
const EGL_GREEN_SIZE: i32 = 0x3023;
const EGL_BLUE_SIZE: i32 = 0x3022;
const EGL_ALPHA_SIZE: i32 = 0x3021;
const EGL_DEPTH_SIZE: i32 = 0x3025;
const EGL_WIDTH: i32 = 0x3057;
const EGL_HEIGHT: i32 = 0x3056;
const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;
const EGL_OPENGL_ES_API: u32 = 0x30A0;
const EGL_VENDOR: i32 = 0x3053;
const EGL_VERSION: i32 = 0x3054;

const GL_VENDOR: u32 = 0x1F00;
const GL_RENDERER: u32 = 0x1F01;
const GL_VERSION: u32 = 0x1F02;
const GL_VERTEX_SHADER: u32 = 0x8B31;
const GL_FRAGMENT_SHADER: u32 = 0x8B30;
const GL_COMPILE_STATUS: u32 = 0x8B81;
const GL_LINK_STATUS: u32 = 0x8B82;
const GL_ARRAY_BUFFER: u32 = 0x8892;
const GL_STATIC_DRAW: u32 = 0x88E4;
const GL_FLOAT: u32 = 0x1406;
const GL_TRIANGLES: u32 = 0x0004;
const GL_DEPTH_TEST: u32 = 0x0B71;
const GL_CULL_FACE: u32 = 0x0B44;
const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
const GL_DEPTH_BUFFER_BIT: u32 = 0x0100;
const GL_RGBA: u32 = 0x1908;
const GL_UNSIGNED_BYTE: u32 = 0x1401;

const VS: &str = "attribute vec3 p; attribute vec3 c; uniform mat4 mvp; varying vec3 v;
void main() { v = c; gl_Position = mvp * vec4(p, 1.0); }\0";
const FS: &str = "precision mediump float; varying vec3 v; void main() { gl_FragColor = vec4(v, 1.0); }\0";

/// A small deterministic generator (the city's own Rng is not linked here).
struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
}

/// Boxes as triangles: position xyz and a shade per face, 36 vertices each.
fn city(n: usize) -> Vec<f32> {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut v = Vec::with_capacity(n * 36 * 6);
    for _ in 0..n {
        let (x, y) = (rng.f() * 2400.0 - 1200.0, rng.f() * 2400.0 - 1200.0);
        let (w, d, h) = (8.0 + rng.f() * 30.0, 8.0 + rng.f() * 30.0, 10.0 + rng.f() * rng.f() * 160.0);
        let t = 0.55 + 0.3 * rng.f();
        let (x0, x1, y0, y1, z0, z1) = (x, x + w, y, y + d, 0.0, h);
        let faces: [([f32; 3], [f32; 3], [f32; 3], [f32; 3], f32); 5] = [
            ([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], 1.0),
            ([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], 0.8),
            ([x1, y1, z0], [x0, y1, z0], [x0, y1, z1], [x1, y1, z1], 0.6),
            ([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], 0.9),
            ([x0, y1, z0], [x0, y0, z0], [x0, y0, z1], [x0, y1, z1], 0.7),
        ];
        for (a, b, c, d, l) in faces {
            for p in [a, b, c, a, c, d] {
                v.extend_from_slice(&p);
                v.extend_from_slice(&[t * l, t * l * 0.97, t * l * 0.95]);
            }
        }
    }
    v
}

/// Column-major perspective * look-at, z up, like the TV renderer's camera.
fn mvp(eye: [f32; 3], at: [f32; 3], aspect: f32) -> [f32; 16] {
    let sub = |a: [f32; 3], b: [f32; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let norm = |a: [f32; 3]| { let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt(); [a[0] / l, a[1] / l, a[2] / l] };
    let cross = |a: [f32; 3], b: [f32; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let f = norm(sub(at, eye));
    let s = norm(cross(f, [0.0, 0.0, 1.0]));
    let u = cross(s, f);
    let view = [s[0], u[0], -f[0], 0.0, s[1], u[1], -f[1], 0.0, s[2], u[2], -f[2], 0.0, -dot(s, eye), -dot(u, eye), dot(f, eye), 1.0];
    let (near, far, fov) = (1.0f32, 4000.0f32, 62.0f32.to_radians());
    let t = 1.0 / (fov / 2.0).tan();
    let proj = [t / aspect, 0.0, 0.0, 0.0, 0.0, t, 0.0, 0.0, 0.0, 0.0, (far + near) / (near - far), -1.0, 0.0, 0.0, 2.0 * far * near / (near - far), 0.0];
    let mut m = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            m[col * 4 + row] = (0..4).map(|k| proj[k * 4 + row] * view[col * 4 + k]).sum();
        }
    }
    m
}

#[repr(C)]
#[derive(Default)]
struct FbVar { xres: u32, yres: u32, xres_virtual: u32, yres_virtual: u32, xoffset: u32, yoffset: u32, bits_per_pixel: u32, grayscale: u32,
    red: [u32; 3], green: [u32; 3], blue: [u32; 3], transp: [u32; 3], rest: [u32; 20] }
#[repr(C)]
#[derive(Default)]
struct FbFix { id: [u8; 16], smem_start: u32, smem_len: u32, type_: u32, type_aux: u32, visual: u32, xpanstep: u16, ypanstep: u16, ywrapstep: u16,
    line_length: u32, mmio_start: u32, mmio_len: u32, accel: u32, capabilities: u16, reserved: [u16; 2] }

const _: [(); 160] = [(); std::mem::size_of::<FbVar>()];
const _: [(); 68] = [(); std::mem::size_of::<FbFix>()];

/// Copy an RGBA frame to /dev/fb0 (32 bpp), if the device is there.
fn show(rgba: &[u8]) -> Result<String, String> {
    let fb = std::fs::OpenOptions::new().read(true).write(true).open("/dev/fb0").map_err(|e| format!("/dev/fb0: {e}"))?;
    let (mut var, mut fix) = (FbVar::default(), FbFix::default());
    unsafe {
        if ioctl(fb.as_raw_fd(), 0x4600, &mut var) != 0 || ioctl(fb.as_raw_fd(), 0x4602, &mut fix) != 0 {
            return Err("fb ioctl failed".into());
        }
    }
    if var.bits_per_pixel != 32 {
        return Err(format!("fb0 is {} bpp", var.bits_per_pixel));
    }
    let pixels = (fix.line_length as usize / 4).checked_mul(var.yres as usize)
        .ok_or_else(|| "framebuffer geometry overflow".to_string())?;
    if fix.line_length % 4 != 0 || pixels == 0 || pixels > fix.smem_len as usize / 4
        || var.xres as usize > fix.line_length as usize / 4 {
        return Err("invalid framebuffer geometry".into());
    }
    for channel in [var.red, var.green, var.blue, var.transp] {
        if channel[1] > 0 && (channel[0] > 24 || channel[1] != 8) {
            return Err("unsupported framebuffer channel layout".into());
        }
    }
    let mem = unsafe { mmap64(std::ptr::null_mut(), fix.smem_len as usize, 3, 1, fb.as_raw_fd(), 0) };
    if mem as isize == -1 {
        return Err("fb mmap failed".into());
    }
    let stride = fix.line_length as usize / 4;
    let px = unsafe { std::slice::from_raw_parts_mut(mem as *mut u32, stride * var.yres as usize) };
    let (w, h) = ((var.xres as usize).min(W as usize), (var.yres as usize).min(H as usize));
    for y in 0..h {
        for x in 0..w {
            // GL reads rows bottom-up.
            let i = ((H as usize - 1 - y) * W as usize + x) * 4;
            let (r, g, b) = (rgba[i] as u32, rgba[i + 1] as u32, rgba[i + 2] as u32);
            px[y * stride + x] = r << var.red[0] | g << var.green[0] | b << var.blue[0] | if var.transp[1] > 0 { 0xff << var.transp[0] } else { 0 };
        }
    }
    Ok(format!("{}x{} {} bpp R{} G{} B{}", var.xres, var.yres, var.bits_per_pixel, var.red[0], var.green[0], var.blue[0]))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let boxes: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(2000);
    let frames: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60);
    println!("GPU_PROBE start pid={} boxes={boxes} frames={frames}", std::process::id());

    let egl = Lib::open("libEGL.so");
    let gles = Lib::open("libGLESv2.so");
    let egl_get_display: unsafe extern "C" fn(*mut c_void) -> EglDisplay = egl.sym("eglGetDisplay");
    let egl_initialize: unsafe extern "C" fn(EglDisplay, *mut i32, *mut i32) -> u32 = egl.sym("eglInitialize");
    let egl_query_string: unsafe extern "C" fn(EglDisplay, i32) -> *const c_char = egl.sym("eglQueryString");
    let egl_choose_config: unsafe extern "C" fn(EglDisplay, *const i32, *mut EglConfig, i32, *mut i32) -> u32 = egl.sym("eglChooseConfig");
    let egl_create_pbuffer: unsafe extern "C" fn(EglDisplay, EglConfig, *const i32) -> EglSurface = egl.sym("eglCreatePbufferSurface");
    let egl_bind_api: unsafe extern "C" fn(u32) -> u32 = egl.sym("eglBindAPI");
    let egl_create_context: unsafe extern "C" fn(EglDisplay, EglConfig, EglContext, *const i32) -> EglContext = egl.sym("eglCreateContext");
    let egl_make_current: unsafe extern "C" fn(EglDisplay, EglSurface, EglSurface, EglContext) -> u32 = egl.sym("eglMakeCurrent");
    let egl_get_error: unsafe extern "C" fn() -> i32 = egl.sym("eglGetError");

    let gl_get_string: unsafe extern "C" fn(u32) -> *const c_char = gles.sym("glGetString");
    let gl_create_shader: unsafe extern "C" fn(u32) -> u32 = gles.sym("glCreateShader");
    let gl_shader_source: unsafe extern "C" fn(u32, i32, *const *const c_char, *const i32) = gles.sym("glShaderSource");
    let gl_compile_shader: unsafe extern "C" fn(u32) = gles.sym("glCompileShader");
    let gl_get_shaderiv: unsafe extern "C" fn(u32, u32, *mut i32) = gles.sym("glGetShaderiv");
    let gl_get_shader_info_log: unsafe extern "C" fn(u32, i32, *mut i32, *mut c_char) = gles.sym("glGetShaderInfoLog");
    let gl_create_program: unsafe extern "C" fn() -> u32 = gles.sym("glCreateProgram");
    let gl_attach_shader: unsafe extern "C" fn(u32, u32) = gles.sym("glAttachShader");
    let gl_link_program: unsafe extern "C" fn(u32) = gles.sym("glLinkProgram");
    let gl_get_programiv: unsafe extern "C" fn(u32, u32, *mut i32) = gles.sym("glGetProgramiv");
    let gl_use_program: unsafe extern "C" fn(u32) = gles.sym("glUseProgram");
    let gl_get_attrib_location: unsafe extern "C" fn(u32, *const c_char) -> i32 = gles.sym("glGetAttribLocation");
    let gl_get_uniform_location: unsafe extern "C" fn(u32, *const c_char) -> i32 = gles.sym("glGetUniformLocation");
    let gl_gen_buffers: unsafe extern "C" fn(i32, *mut u32) = gles.sym("glGenBuffers");
    let gl_bind_buffer: unsafe extern "C" fn(u32, u32) = gles.sym("glBindBuffer");
    let gl_buffer_data: unsafe extern "C" fn(u32, isize, *const c_void, u32) = gles.sym("glBufferData");
    let gl_vertex_attrib_pointer: unsafe extern "C" fn(u32, i32, u32, u8, i32, *const c_void) = gles.sym("glVertexAttribPointer");
    let gl_enable_vertex_attrib_array: unsafe extern "C" fn(u32) = gles.sym("glEnableVertexAttribArray");
    let gl_enable: unsafe extern "C" fn(u32) = gles.sym("glEnable");
    let gl_viewport: unsafe extern "C" fn(i32, i32, i32, i32) = gles.sym("glViewport");
    let gl_clear_color: unsafe extern "C" fn(f32, f32, f32, f32) = gles.sym("glClearColor");
    let gl_clear: unsafe extern "C" fn(u32) = gles.sym("glClear");
    let gl_uniform_matrix4fv: unsafe extern "C" fn(i32, i32, u8, *const f32) = gles.sym("glUniformMatrix4fv");
    let gl_draw_arrays: unsafe extern "C" fn(u32, i32, i32) = gles.sym("glDrawArrays");
    let gl_finish: unsafe extern "C" fn() = gles.sym("glFinish");
    let gl_read_pixels: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, *mut c_void) = gles.sym("glReadPixels");
    let gl_get_error: unsafe extern "C" fn() -> u32 = gles.sym("glGetError");

    let text = |p: *const c_char| if p.is_null() { "(null)".to_string() } else { unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned() };
    unsafe {
        let display = egl_get_display(std::ptr::null_mut());
        assert!(!display.is_null(), "eglGetDisplay failed: 0x{:x}", egl_get_error());
        let (mut major, mut minor) = (0, 0);
        assert_eq!(egl_initialize(display, &mut major, &mut minor), 1, "eglInitialize failed: 0x{:x}", egl_get_error());
        println!("GPU_PROBE egl {major}.{minor} vendor={:?} version={:?}", text(egl_query_string(display, EGL_VENDOR)), text(egl_query_string(display, EGL_VERSION)));
        let attribs = [EGL_SURFACE_TYPE, EGL_PBUFFER_BIT, EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT, EGL_RED_SIZE, 8, EGL_GREEN_SIZE, 8,
                       EGL_BLUE_SIZE, 8, EGL_ALPHA_SIZE, 8, EGL_DEPTH_SIZE, 16, EGL_NONE];
        let mut config: EglConfig = std::ptr::null_mut();
        let mut count = 0;
        assert_eq!(egl_choose_config(display, attribs.as_ptr(), &mut config, 1, &mut count), 1, "eglChooseConfig failed: 0x{:x}", egl_get_error());
        assert!(count > 0, "no pbuffer-capable GLES2 config");
        let surface = egl_create_pbuffer(display, config, [EGL_WIDTH, W, EGL_HEIGHT, H, EGL_NONE].as_ptr());
        assert!(!surface.is_null(), "eglCreatePbufferSurface failed: 0x{:x}", egl_get_error());
        assert_eq!(egl_bind_api(EGL_OPENGL_ES_API), 1);
        let context = egl_create_context(display, config, std::ptr::null_mut(), [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE].as_ptr());
        assert!(!context.is_null(), "eglCreateContext failed: 0x{:x}", egl_get_error());
        assert_eq!(egl_make_current(display, surface, surface, context), 1, "eglMakeCurrent failed: 0x{:x}", egl_get_error());
        println!("GPU_PROBE gl vendor={:?} renderer={:?} version={:?}", text(gl_get_string(GL_VENDOR)), text(gl_get_string(GL_RENDERER)), text(gl_get_string(GL_VERSION)));

        let shader = |kind: u32, src: &str| {
            let s = gl_create_shader(kind);
            let p = src.as_ptr() as *const c_char;
            gl_shader_source(s, 1, &p, std::ptr::null());
            gl_compile_shader(s);
            let mut ok = 0;
            gl_get_shaderiv(s, GL_COMPILE_STATUS, &mut ok);
            if ok == 0 {
                let mut log = vec![0 as c_char; 1024];
                let mut n = 0;
                gl_get_shader_info_log(s, 1024, &mut n, log.as_mut_ptr());
                panic!("shader: {}", text(log.as_ptr()));
            }
            s
        };
        let program = gl_create_program();
        gl_attach_shader(program, shader(GL_VERTEX_SHADER, VS));
        gl_attach_shader(program, shader(GL_FRAGMENT_SHADER, FS));
        gl_link_program(program);
        let mut linked = 0;
        gl_get_programiv(program, GL_LINK_STATUS, &mut linked);
        assert_eq!(linked, 1, "program link failed");
        gl_use_program(program);

        let verts = city(boxes);
        let mut vbo = 0;
        gl_gen_buffers(1, &mut vbo);
        gl_bind_buffer(GL_ARRAY_BUFFER, vbo);
        gl_buffer_data(GL_ARRAY_BUFFER, (verts.len() * 4) as isize, verts.as_ptr() as *const c_void, GL_STATIC_DRAW);
        let p = gl_get_attrib_location(program, c"p".as_ptr()) as u32;
        let c = gl_get_attrib_location(program, c"c".as_ptr()) as u32;
        gl_vertex_attrib_pointer(p, 3, GL_FLOAT, 0, 24, std::ptr::null());
        gl_vertex_attrib_pointer(c, 3, GL_FLOAT, 0, 24, 12 as *const c_void);
        gl_enable_vertex_attrib_array(p);
        gl_enable_vertex_attrib_array(c);
        let u = gl_get_uniform_location(program, c"mvp".as_ptr());
        gl_enable(GL_DEPTH_TEST);
        gl_enable(GL_CULL_FACE);
        gl_viewport(0, 0, W, H);
        gl_clear_color(0.36, 0.56, 0.86, 1.0);
        let vertex_count = (verts.len() / 6) as i32;
        println!("GPU_PROBE scene {boxes} boxes, {vertex_count} vertices, {} MB", verts.len() * 4 / (1 << 20));

        let t0 = Instant::now();
        for i in 0..frames {
            let a = i as f32 * 0.02;
            let m = mvp([-900.0 + 1200.0 * a.sin().abs(), -1100.0, 120.0], [a.cos() * 400.0, 300.0, 60.0], W as f32 / H as f32);
            gl_uniform_matrix4fv(u, 1, 0, m.as_ptr());
            gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            gl_draw_arrays(GL_TRIANGLES, 0, vertex_count);
            gl_finish();
        }
        let per_frame = t0.elapsed().as_secs_f64() * 1e3 / frames as f64;
        let err = gl_get_error();
        println!("GPU_PROBE frame {per_frame:.2} ms ({:.1} fps) glGetError=0x{err:x}", 1e3 / per_frame);

        let mut rgba = vec![0u8; (W * H * 4) as usize];
        let t1 = Instant::now();
        gl_read_pixels(0, 0, W, H, GL_RGBA, GL_UNSIGNED_BYTE, rgba.as_mut_ptr() as *mut c_void);
        let readback = t1.elapsed().as_secs_f64() * 1e3;
        let lit = rgba.chunks(4).filter(|p| p[0] != 92 || p[1] != 143).count();
        println!("GPU_PROBE readpixels {readback:.2} ms, {lit} of {} pixels are not sky", W * H);
        assert!(lit > 10000, "the frame has no buildings");
        std::fs::File::create("/tmp/gpu-frame.rgba").and_then(|mut f| f.write_all(&rgba)).expect("write frame");
        match show(&rgba) {
            Ok(info) => println!("GPU_PROBE shown on /dev/fb0 ({info})"),
            Err(e) => println!("GPU_PROBE not shown: {e}"),
        }
        println!("GPU_PROBE_PASS egl pbuffer gles2 draw readback");
    }
}
