//! EGL and OpenGL ES 2 through Android's loaders, bound at run time with dlopen
//! so nothing but libdl is linked: libEGL.so and libGLESv2.so pick
//! /vendor/lib/egl/libGLES_mali.so. The surface is an off-screen pbuffer; no
//! window system is involved (see tools/gpu/README.txt).

use std::ffi::{c_char, c_void, CStr, CString};

extern "C" {
    fn dlopen(name: *const c_char, flags: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

fn open(name: &str) -> *mut c_void {
    let c = CString::new(name).unwrap();
    let h = unsafe { dlopen(c.as_ptr(), 2) };
    if h.is_null() {
        let e = unsafe { dlerror() };
        panic!("dlopen {name}: {}", if e.is_null() { "unknown".into() } else { unsafe { CStr::from_ptr(e) }.to_string_lossy() });
    }
    h
}

fn sym<T: Copy>(handle: *mut c_void, name: &str) -> T {
    let c = CString::new(name).unwrap();
    let p = unsafe { dlsym(handle, c.as_ptr()) };
    assert!(!p.is_null(), "missing symbol {name}");
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<*mut c_void>());
    unsafe { std::mem::transmute_copy(&p) }
}

pub type Ptr = *mut c_void;
pub const EGL_NONE: i32 = 0x3038;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_LINES: u32 = 0x0001;
pub const GL_DEPTH_TEST: u32 = 0x0B71;
pub const GL_CULL_FACE: u32 = 0x0B44;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x0100;
pub const GL_RGBA: u32 = 0x1908;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;

#[allow(non_snake_case)]
pub struct Gl {
    pub GetString: unsafe extern "C" fn(u32) -> *const c_char,
    pub CreateShader: unsafe extern "C" fn(u32) -> u32,
    pub ShaderSource: unsafe extern "C" fn(u32, i32, *const *const c_char, *const i32),
    pub CompileShader: unsafe extern "C" fn(u32),
    pub GetShaderiv: unsafe extern "C" fn(u32, u32, *mut i32),
    pub GetShaderInfoLog: unsafe extern "C" fn(u32, i32, *mut i32, *mut c_char),
    pub CreateProgram: unsafe extern "C" fn() -> u32,
    pub AttachShader: unsafe extern "C" fn(u32, u32),
    pub LinkProgram: unsafe extern "C" fn(u32),
    pub GetProgramiv: unsafe extern "C" fn(u32, u32, *mut i32),
    pub UseProgram: unsafe extern "C" fn(u32),
    pub GetAttribLocation: unsafe extern "C" fn(u32, *const c_char) -> i32,
    pub GetUniformLocation: unsafe extern "C" fn(u32, *const c_char) -> i32,
    pub GenBuffers: unsafe extern "C" fn(i32, *mut u32),
    pub BindBuffer: unsafe extern "C" fn(u32, u32),
    pub BufferData: unsafe extern "C" fn(u32, isize, *const c_void, u32),
    pub VertexAttribPointer: unsafe extern "C" fn(u32, i32, u32, u8, i32, *const c_void),
    pub EnableVertexAttribArray: unsafe extern "C" fn(u32),
    pub Enable: unsafe extern "C" fn(u32),
    pub Disable: unsafe extern "C" fn(u32),
    pub Viewport: unsafe extern "C" fn(i32, i32, i32, i32),
    pub ClearColor: unsafe extern "C" fn(f32, f32, f32, f32),
    pub Clear: unsafe extern "C" fn(u32),
    pub UniformMatrix4fv: unsafe extern "C" fn(i32, i32, u8, *const f32),
    pub Uniform3f: unsafe extern "C" fn(i32, f32, f32, f32),
    pub Uniform1f: unsafe extern "C" fn(i32, f32),
    pub DrawArrays: unsafe extern "C" fn(u32, i32, i32),
    pub LineWidth: unsafe extern "C" fn(f32),
    pub Finish: unsafe extern "C" fn(),
    pub ReadPixels: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, *mut c_void),
    pub GetError: unsafe extern "C" fn() -> u32,
}

/// An initialised GLES 2 context on a pbuffer of the given size.
pub struct Context {
    pub gl: Gl,
    pub vendor: String,
    pub renderer: String,
}

pub fn text(p: *const c_char) -> String {
    if p.is_null() { "(null)".into() } else { unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned() }
}

impl Context {
    pub fn new(width: i32, height: i32) -> Context {
        let egl = open("libEGL.so");
        let gles = open("libGLESv2.so");
        let get_display: unsafe extern "C" fn(Ptr) -> Ptr = sym(egl, "eglGetDisplay");
        let initialize: unsafe extern "C" fn(Ptr, *mut i32, *mut i32) -> u32 = sym(egl, "eglInitialize");
        let choose_config: unsafe extern "C" fn(Ptr, *const i32, *mut Ptr, i32, *mut i32) -> u32 = sym(egl, "eglChooseConfig");
        let create_pbuffer: unsafe extern "C" fn(Ptr, Ptr, *const i32) -> Ptr = sym(egl, "eglCreatePbufferSurface");
        let bind_api: unsafe extern "C" fn(u32) -> u32 = sym(egl, "eglBindAPI");
        let create_context: unsafe extern "C" fn(Ptr, Ptr, Ptr, *const i32) -> Ptr = sym(egl, "eglCreateContext");
        let make_current: unsafe extern "C" fn(Ptr, Ptr, Ptr, Ptr) -> u32 = sym(egl, "eglMakeCurrent");
        let get_error: unsafe extern "C" fn() -> i32 = sym(egl, "eglGetError");
        unsafe {
            let display = get_display(std::ptr::null_mut());
            assert!(!display.is_null(), "eglGetDisplay failed: 0x{:x}", get_error());
            let (mut major, mut minor) = (0, 0);
            assert_eq!(initialize(display, &mut major, &mut minor), 1, "eglInitialize failed: 0x{:x}", get_error());
            // Surface type pbuffer, GLES 2, RGBA8 with a 16-bit depth buffer.
            let attribs = [0x3033, 1, 0x3040, 4, 0x3024, 8, 0x3023, 8, 0x3022, 8, 0x3021, 8, 0x3025, 16, EGL_NONE];
            let mut config: Ptr = std::ptr::null_mut();
            let mut count = 0;
            assert_eq!(choose_config(display, attribs.as_ptr(), &mut config, 1, &mut count), 1, "eglChooseConfig failed: 0x{:x}", get_error());
            assert!(count > 0, "no pbuffer-capable GLES2 config");
            let surface = create_pbuffer(display, config, [0x3057, width, 0x3056, height, EGL_NONE].as_ptr());
            assert!(!surface.is_null(), "eglCreatePbufferSurface failed: 0x{:x}", get_error());
            assert_eq!(bind_api(0x30A0), 1);
            let context = create_context(display, config, std::ptr::null_mut(), [0x3098, 2, EGL_NONE].as_ptr());
            assert!(!context.is_null(), "eglCreateContext failed: 0x{:x}", get_error());
            assert_eq!(make_current(display, surface, surface, context), 1, "eglMakeCurrent failed: 0x{:x}", get_error());
        }
        let gl = Gl {
            GetString: sym(gles, "glGetString"), CreateShader: sym(gles, "glCreateShader"), ShaderSource: sym(gles, "glShaderSource"),
            CompileShader: sym(gles, "glCompileShader"), GetShaderiv: sym(gles, "glGetShaderiv"), GetShaderInfoLog: sym(gles, "glGetShaderInfoLog"),
            CreateProgram: sym(gles, "glCreateProgram"), AttachShader: sym(gles, "glAttachShader"), LinkProgram: sym(gles, "glLinkProgram"),
            GetProgramiv: sym(gles, "glGetProgramiv"), UseProgram: sym(gles, "glUseProgram"), GetAttribLocation: sym(gles, "glGetAttribLocation"),
            GetUniformLocation: sym(gles, "glGetUniformLocation"), GenBuffers: sym(gles, "glGenBuffers"), BindBuffer: sym(gles, "glBindBuffer"),
            BufferData: sym(gles, "glBufferData"), VertexAttribPointer: sym(gles, "glVertexAttribPointer"),
            EnableVertexAttribArray: sym(gles, "glEnableVertexAttribArray"), Enable: sym(gles, "glEnable"), Disable: sym(gles, "glDisable"),
            Viewport: sym(gles, "glViewport"), ClearColor: sym(gles, "glClearColor"), Clear: sym(gles, "glClear"),
            UniformMatrix4fv: sym(gles, "glUniformMatrix4fv"), Uniform3f: sym(gles, "glUniform3f"), Uniform1f: sym(gles, "glUniform1f"),
            DrawArrays: sym(gles, "glDrawArrays"), LineWidth: sym(gles, "glLineWidth"), Finish: sym(gles, "glFinish"),
            ReadPixels: sym(gles, "glReadPixels"), GetError: sym(gles, "glGetError"),
        };
        let (vendor, renderer) = unsafe { (text((gl.GetString)(0x1F00)), text((gl.GetString)(0x1F01))) };
        Context { gl, vendor, renderer }
    }

    /// Compile and link a vertex and fragment shader; panics with the driver's log on failure.
    pub fn program(&self, vs: &str, fs: &str) -> u32 {
        let gl = &self.gl;
        let shader = |kind: u32, src: &str| unsafe {
            let s = (gl.CreateShader)(kind);
            let c = CString::new(src).unwrap();
            let p = c.as_ptr();
            (gl.ShaderSource)(s, 1, &p, std::ptr::null());
            (gl.CompileShader)(s);
            let mut ok = 0;
            (gl.GetShaderiv)(s, GL_COMPILE_STATUS, &mut ok);
            if ok == 0 {
                let mut log = vec![0 as c_char; 2048];
                let mut n = 0;
                (gl.GetShaderInfoLog)(s, 2048, &mut n, log.as_mut_ptr());
                panic!("shader: {}", text(log.as_ptr()));
            }
            s
        };
        unsafe {
            let program = (gl.CreateProgram)();
            (gl.AttachShader)(program, shader(GL_VERTEX_SHADER, vs));
            (gl.AttachShader)(program, shader(GL_FRAGMENT_SHADER, fs));
            (gl.LinkProgram)(program);
            let mut linked = 0;
            (gl.GetProgramiv)(program, GL_LINK_STATUS, &mut linked);
            assert_eq!(linked, 1, "program link failed");
            program
        }
    }
}
