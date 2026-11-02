use platform::fb::{Bitfield, VarInfo};

#[test]
fn driver_formats_and_opaque_alpha() {
    let field = |offset, length| Bitfield { offset, length, msb_right: 0 };
    let mut v = VarInfo { red: field(16, 8), green: field(8, 8), blue: field(0, 8),
                          transp: field(24, 8), ..VarInfo::default() };
    assert_eq!(v.pixel(0x12, 0x34, 0x56), 0xff123456);
    assert_eq!(v.dim(0xff123456), 0xff091a2b);
    v.transp.length = 0;
    assert_eq!(v.pixel(0x12, 0x34, 0x56), 0x00123456);
    v.red.offset = 0;
    v.blue.offset = 16;
    assert_eq!(v.pixel(0x12, 0x34, 0x56), 0x00563412);
    v.red = field(11, 5);
    v.green = field(5, 6);
    v.blue = field(0, 5);
    assert_eq!(v.pixel(255, 0, 0), 0xf800);
    assert_eq!(v.pixel(0, 255, 0), 0x07e0);
    assert_eq!(v.pixel(0, 0, 255), 0x001f);
    assert_eq!(v.dim(0xf800), 0x7800);
}
