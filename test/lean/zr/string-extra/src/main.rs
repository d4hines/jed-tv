use std::ffi::{c_char, c_void, CString};
type Object = *mut c_void;
extern "C" {
    fn test_init();
    fn test_retain(o: Object);
    fn test_release(o: Object);
    fn test_text(p: *const u8, n: usize) -> Object;
    fn test_data(o: Object) -> *const u8;
    fn test_size(o: Object) -> usize;
    fn test_cons(h: Object, t: Object) -> Object;
    fn test_int(p: *const c_char, neg: u8) -> Object;
    fn test_join(s: Object, xs: Object) -> Object;
    fn test_repr(n: Object) -> Object;
    fn test_get(s: Object, i: usize) -> u32;
    fn test_next(s: Object, i: usize) -> usize;
}
unsafe fn text(s: &str) -> Object {
    test_text(s.as_ptr(), s.len())
}
unsafe fn borrowed(o: Object) -> String {
    String::from_utf8(std::slice::from_raw_parts(test_data(o), test_size(o)).to_vec()).unwrap()
}
unsafe fn owned(o: Object) -> String {
    let s = borrowed(o);
    test_release(o);
    s
}
fn joins() {
    for sep in ["", ",", "é🙂", "\0"] {
        for pieces in [
            vec![],
            vec![""],
            vec!["a"],
            vec!["a", "", "é", "🙂", "\0tail"],
        ] {
            unsafe {
                let separator = text(sep);
                let mut list = 1usize as Object;
                for part in pieces.iter().rev() {
                    list = test_cons(text(part), list);
                }
                test_retain(list);
                assert_eq!(owned(test_join(separator, list)), pieces.join(sep));
                assert_eq!(owned(test_join(separator, list)), pieces.join(sep));
                assert_eq!(borrowed(separator), sep);
                test_release(separator);
            }
        }
    }
    unsafe {
        let sep = text("shared");
        test_retain(sep);
        test_retain(sep);
        let xs = test_cons(sep, test_cons(sep, 1usize as Object));
        assert_eq!(owned(test_join(sep, xs)), "sharedsharedshared");
        assert_eq!(borrowed(sep), "shared");
        test_release(sep);
    }
}
fn ints() {
    let values = (-4096..4096).map(i128::from).chain([
        i128::MIN,
        i128::MIN + 1,
        -(1i128 << 100),
        -(1i128 << 64),
        i64::MIN as i128,
        -(1i128 << 31),
        -(1i128 << 30) - 1,
        1i128 << 30,
        1i128 << 31,
        i64::MAX as i128,
        1i128 << 64,
        1i128 << 100,
        i128::MAX,
    ]);
    for n in values {
        let digits = CString::new(if n < 0 {
            (-(n + 1)).to_string()
        } else {
            n.to_string()
        })
        .unwrap();
        unsafe {
            let o = test_int(digits.as_ptr(), u8::from(n < 0));
            assert_eq!(owned(test_repr(o)), n.to_string());
            assert_eq!(owned(test_repr(o)), n.to_string());
            test_release(o);
        }
    }
}
fn utf8() {
    let mut count = 0;
    for start in (0..=0x10ffff).step_by(4096) {
        let chars: Vec<_> = (start..=(start + 4095).min(0x10ffff))
            .filter_map(char::from_u32)
            .collect();
        let s: String = chars.iter().collect();
        unsafe {
            let obj = text(&s);
            let mut at = 0;
            for c in chars {
                assert_eq!(test_get(obj, at), c as u32);
                assert_eq!(test_next(obj, at), at + c.len_utf8());
                at += c.len_utf8();
                count += 1;
            }
            assert_eq!(test_get(obj, at), 0);
            assert_eq!(test_next(obj, at), at + 1);
            assert_eq!(borrowed(obj), s);
            test_release(obj);
        }
    }
    assert_eq!(count, 1_112_064);
}
fn main() {
    unsafe { test_init() };
    joins();
    ints();
    utf8();
    println!("PASS: StringExtra shared intercalate, signed128 repr, all1112064 Unicode scalar get/next and borrowed ownership");
}
