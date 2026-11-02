//! Native replacement for AudioBridge.java. The PCM wire protocol is unchanged.
use std::ffi::{c_char, c_void, CStr, CString};
use std::io;
use std::sync::Arc;
mod protocol;
use std::time::{Duration, Instant};

type Handle = *mut c_void;
extern "C" {
    fn dlopen(path: *const c_char, flags: i32) -> Handle;
    fn dlsym(handle: Handle, name: *const c_char) -> Handle;
    fn dlerror() -> *const c_char;
}
type Set = unsafe extern "C" fn(Handle, i32);
type Op = unsafe extern "C" fn(Handle) -> i32;
type Get = unsafe extern "C" fn(Handle) -> i32;
type Counter = unsafe extern "C" fn(Handle) -> i64;
struct Api {
    create: unsafe extern "C" fn(*mut Handle) -> i32,
    delete: Op,
    sample_rate: Set,
    channels: Set,
    format: Set,
    sharing: Set,
    performance: Set,
    usage: Set,
    direction: Set,
    open: unsafe extern "C" fn(Handle, *mut Handle) -> i32,
    start: Op,
    stop: Op,
    close: Op,
    write: unsafe extern "C" fn(Handle, *const c_void, i32, i64) -> i32,
    read_frames: Counter,
    get_rate: Get,
    get_channels: Get,
    get_format: Get,
    capacity: Get,
    burst: Get,
}
fn fail(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}
unsafe fn symbol<T: Copy>(lib: Handle, name: &str) -> io::Result<T> {
    let name = CString::new(name).unwrap();
    let pointer = dlsym(lib, name.as_ptr());
    if pointer.is_null() {
        return Err(fail(format!("missing {}", name.to_string_lossy())));
    }
    Ok(std::mem::transmute_copy(&pointer))
}
fn checked(status: i32, stage: &str) -> io::Result<()> {
    if status < 0 {
        Err(fail(format!("{stage}: AAudio {status}")))
    } else {
        Ok(())
    }
}
impl Api {
    fn load() -> io::Result<Self> {
        unsafe {
            let lib = dlopen(c"libaaudio.so".as_ptr(), 2);
            if lib.is_null() {
                let text = dlerror();
                return Err(fail(if text.is_null() {
                    "dlopen failed".into()
                } else {
                    CStr::from_ptr(text).to_string_lossy().into_owned()
                }));
            }
            let mmap: unsafe extern "C" fn(i32) -> i32 = symbol(lib, "AAudio_setMMapPolicy")?;
            checked(mmap(1), "disable MMAP")?;
            Ok(Self {
                create: symbol(lib, "AAudio_createStreamBuilder")?,
                delete: symbol(lib, "AAudioStreamBuilder_delete")?,
                sample_rate: symbol(lib, "AAudioStreamBuilder_setSampleRate")?,
                channels: symbol(lib, "AAudioStreamBuilder_setChannelCount")?,
                format: symbol(lib, "AAudioStreamBuilder_setFormat")?,
                sharing: symbol(lib, "AAudioStreamBuilder_setSharingMode")?,
                performance: symbol(lib, "AAudioStreamBuilder_setPerformanceMode")?,
                usage: symbol(lib, "AAudioStreamBuilder_setUsage")?,
                direction: symbol(lib, "AAudioStreamBuilder_setDirection")?,
                open: symbol(lib, "AAudioStreamBuilder_openStream")?,
                start: symbol(lib, "AAudioStream_requestStart")?,
                stop: symbol(lib, "AAudioStream_requestStop")?,
                close: symbol(lib, "AAudioStream_close")?,
                write: symbol(lib, "AAudioStream_write")?,
                read_frames: symbol(lib, "AAudioStream_getFramesRead")?,
                get_rate: symbol(lib, "AAudioStream_getSampleRate")?,
                get_channels: symbol(lib, "AAudioStream_getChannelCount")?,
                get_format: symbol(lib, "AAudioStream_getFormat")?,
                capacity: symbol(lib, "AAudioStream_getBufferCapacityInFrames")?,
                burst: symbol(lib, "AAudioStream_getFramesPerBurst")?,
            })
        }
    }
}
struct Track {
    api: Arc<Api>,
    handle: Handle,
    written: u64,
    channels: u32,
}
impl Track {
    fn open(api: Arc<Api>, rate: u32, channels: u32) -> io::Result<Self> {
        unsafe {
            let mut builder = std::ptr::null_mut();
            checked((api.create)(&mut builder), "builder")?;
            (api.sample_rate)(builder, rate as i32);
            (api.channels)(builder, channels as i32);
            (api.format)(builder, 1);
            (api.sharing)(builder, 1);
            (api.performance)(builder, 10);
            (api.usage)(builder, 1);
            (api.direction)(builder, 0);
            let mut handle = std::ptr::null_mut();
            let status = (api.open)(builder, &mut handle);
            (api.delete)(builder);
            checked(status, "open")?;
            let track = Self {
                api,
                handle,
                written: 0,
                channels,
            };
            if (track.api.get_rate)(handle) != rate as i32
                || (track.api.get_channels)(handle) != channels as i32
                || (track.api.get_format)(handle) != 1
            {
                return Err(fail("AAudio changed PCM format"));
            }
            checked((track.api.start)(handle), "start")?;
            println!(
                "RUST_AUDIO open rate={rate} channels={channels} capacity={} burst={}",
                (track.api.capacity)(handle),
                (track.api.burst)(handle)
            );
            Ok(track)
        }
    }
    fn played(&self) -> io::Result<u64> {
        let n = unsafe { (self.api.read_frames)(self.handle) };
        if n < 0 {
            Err(fail(format!("frame counter: {n}")))
        } else {
            Ok((n as u64).min(self.written))
        }
    }
    fn write(&mut self, samples: &[i16]) -> io::Result<()> {
        let mut frames = 0;
        let count = samples.len() / self.channels as usize;
        let deadline = Instant::now() + Duration::from_secs(5);
        while frames < count {
            if Instant::now() >= deadline {
                return Err(fail("audio write deadline"));
            }
            // Android 11's legacy backend does not enforce positive timeouts.
            let n = unsafe {
                (self.api.write)(
                    self.handle,
                    samples[frames * self.channels as usize..].as_ptr().cast(),
                    (count - frames) as i32,
                    0,
                )
            };
            if n < 0 {
                return Err(fail(format!("write: AAudio {n}")));
            }
            if n == 0 {
                if Instant::now() >= deadline {
                    return Err(fail("audio write deadline"));
                }
                std::thread::sleep(Duration::from_millis(2));
            } else {
                if n as usize > count - frames {
                    return Err(fail("AAudio wrote more than requested"));
                }
                frames += n as usize;
                self.written += n as u64;
            }
        }
        Ok(())
    }
}
impl Drop for Track {
    fn drop(&mut self) {
        unsafe {
            (self.api.stop)(self.handle);
            (self.api.close)(self.handle);
        }
    }
}
fn initialize_policy() -> io::Result<()> {
    // Explicit parity operation with the previous Java prepareAudio(), not a
    // default startup side effect. The global software gain remains untouched.
    unsafe {
        let lib = dlopen(c"libaudioclient.so".as_ptr(), 2);
        if lib.is_null() {
            return Err(fail("dlopen libaudioclient for policy"));
        }
        let init: unsafe extern "C" fn(i32, i32, i32) -> i32 = symbol(
            lib,
            "_ZN7android11AudioSystem16initStreamVolumeE19audio_stream_type_tii",
        )?;
        // Android 14 names audio_devices_t as an enum in this exported ABI;
        // Android 11 uses its unsigned-int typedef. Both pass one 32-bit word.
        let volume: unsafe extern "C" fn(i32, i32, u32) -> i32 = symbol(
            lib, "_ZN7android11AudioSystem20setStreamVolumeIndexE19audio_stream_type_ti15audio_devices_t",
        ).or_else(|_| symbol(
            lib, "_ZN7android11AudioSystem20setStreamVolumeIndexE19audio_stream_type_tij",
        ))?;
        checked(init(3, 0, 100), "initialize media volume range")?;
        checked(volume(3, 30, 2), "set speaker policy index")?;
        println!("RUST_AUDIO policy initialized: stream3 range0..100 speaker2 index30; software gain unchanged");
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut port = 8878;
    let mut init_policy = false;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--port" => {
                port = args
                    .next()
                    .and_then(|v| v.parse::<u16>().ok())
                    .ok_or_else(|| fail("expected --port NUMBER"))?
            }
            "--init-policy" => init_policy = true,
            "--help" => {
                println!("Usage: rust-audio-bridge [--port PORT] [--init-policy]");
                return Ok(());
            }
            _ => return Err(fail("unknown option")),
        }
    }
    let api = Arc::new(Api::load()?);
    if init_policy {
        initialize_policy()?;
    }
    protocol::serve(AAudioBackend { api }, port)
}
struct AAudioBackend {
    api: Arc<Api>,
}
impl protocol::Backend for AAudioBackend {
    type Track = Track;
    fn supports(&self, rate: u32, channels: u32) -> bool {
        (8000..=96000).contains(&rate) && (1..=2).contains(&channels)
    }
    fn open(&self, rate: u32, channels: u32) -> io::Result<Track> {
        Track::open(self.api.clone(), rate, channels)
    }
    fn name(&self) -> &'static str {
        "AAudio legacy/shared"
    }
}
impl protocol::Stream for Track {
    fn write(&mut self, samples: &[i16]) -> io::Result<()> {
        Track::write(self, samples)
    }
    fn written(&self) -> u64 {
        self.written
    }
    fn played(&self) -> io::Result<u64> {
        Track::played(self)
    }
}
