//! Owns the Lean session on the app's single main thread. No socket or server.
use crate::game::{Bar, Mood, Opt, Screen};
use std::{cell::RefCell, ffi::c_void};
type Object = *mut c_void;
extern "C" {
    fn initialize_rpgTv_RpgTv_Engine(builtin: u8) -> Object;
    fn tv_rpg_new(seed: u32) -> Object;
    fn tv_rpg_choose(session: Object, index: u32) -> Object;
    fn tv_rpg_render(session: Object) -> Object;
    fn tv_rpg_io_unit() -> Object;
    fn lean_dec_ref_cold(value: Object);
    fn lean_io_mark_end_initialization();
}
#[repr(C)]
struct Header {
    rc: i32,
    size: u16,
    other: u8,
    tag: u8,
}
#[repr(C)]
struct LeanString {
    header: Header,
    size: usize,
    capacity: usize,
    length: usize,
}
unsafe fn string(value: Object) -> String {
    let header = &*value.cast::<LeanString>();
    assert_eq!(header.header.tag, 249);
    let bytes = std::slice::from_raw_parts(
        value.cast::<u8>().add(std::mem::size_of::<LeanString>()),
        header.size - 1,
    );
    std::str::from_utf8(bytes)
        .expect("Lean UTF-8 string")
        .to_owned()
}
unsafe fn retain(value: Object) {
    if value as usize & 1 == 0 {
        let h = &mut *value.cast::<Header>();
        if h.rc > 0 {
            h.rc += 1;
        }
    }
}
unsafe fn release(value: Object) {
    if value as usize & 1 == 0 {
        let h = &mut *value.cast::<Header>();
        if h.rc > 1 {
            h.rc -= 1;
        } else if h.rc != 0 {
            lean_dec_ref_cold(value);
        }
    }
}
unsafe fn unit(value: Object) {
    assert!(!value.is_null() && value as usize & 1 == 0);
    assert_eq!(
        (*value.cast::<Header>()).tag,
        0,
        "Lean engine effect failed"
    );
    release(value);
}
thread_local! {static VIEW:RefCell<Screen> = RefCell::new(Screen::default());}
pub struct Engine {
    state: Object,
}
impl Engine {
    pub fn new(seed: u32) -> Self {
        unsafe {
            unit(initialize_rpgTv_RpgTv_Engine(1));
            lean_io_mark_end_initialization();
        }
        Self {
            state: unsafe { tv_rpg_new(seed) },
        }
    }
    pub fn choose(&mut self, index: usize) {
        self.state = unsafe { tv_rpg_choose(self.state, index as u32) };
    }
    pub fn screen(&self) -> Screen {
        unsafe {
            retain(self.state);
            unit(tv_rpg_render(self.state));
        }
        VIEW.with(|v| v.borrow().clone())
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            release(self.state);
        }
    }
}
#[no_mangle]
extern "C" fn tv_rpg_header(
    phase: Object,
    adventure: Object,
    heading: Object,
    body: Object,
    picture: Object,
    mood: u32,
) -> Object {
    VIEW.with(|v| unsafe {
        let p = string(picture);
        *v.borrow_mut() = Screen {
            phase: string(phase),
            adventure: string(adventure),
            heading: string(heading),
            body: string(body),
            picture: if p.is_empty() { None } else { Some(p) },
            mood: match mood {
                1 => Mood::Dead,
                2 => Mood::Ended,
                _ => Mood::Normal,
            },
            ..Screen::default()
        };
    });
    unsafe { tv_rpg_io_unit() }
}
#[no_mangle]
extern "C" fn tv_rpg_bar(kind: u32, name: Object, hp: i32, maximum: u32, alive: u32) -> Object {
    VIEW.with(|v| unsafe {
        let b = Bar {
            name: string(name),
            hp: hp as i64,
            max: maximum as i64,
            alive: alive != 0,
        };
        if kind == 0 {
            v.borrow_mut().hero = b;
        } else {
            v.borrow_mut().foes.push(b);
        }
    });
    unsafe { tv_rpg_io_unit() }
}
#[no_mangle]
extern "C" fn tv_rpg_side_title(value: Object) -> Object {
    VIEW.with(|v| unsafe { v.borrow_mut().side_title = string(value) });
    unsafe { tv_rpg_io_unit() }
}
#[no_mangle]
extern "C" fn tv_rpg_side(value: Object) -> Object {
    VIEW.with(|v| unsafe { v.borrow_mut().side.push(string(value)) });
    unsafe { tv_rpg_io_unit() }
}
#[no_mangle]
extern "C" fn tv_rpg_option(label: Object, help: Object) -> Object {
    VIEW.with(|v| unsafe {
        v.borrow_mut().options.push(Opt {
            label: string(label),
            help: string(help),
        })
    });
    unsafe { tv_rpg_io_unit() }
}
