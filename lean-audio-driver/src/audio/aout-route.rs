//! Exact V109 MI AOUT wire adapter for the existing speaker output: one
//! function per request, raw values in and out. Never opens or closes an
//! output stream or initializes hardware. Whether to connect, what to restore
//! and what counts as a valid handle is decided in Lean (src/lean/TV/Audio).
use crate::mi_pcm::{call, Transport};
use std::io;

pub const GET_HANDLE: u32 = 0xc0081116;
pub const GET_COUNT: u32 = 0xc010111e;
pub const GET_CONNECTED: u32 = 0xc018111f;
pub const CONNECT: u32 = 0xc00c111c;
pub const DISCONNECT: u32 = 0xc008111d;
pub const GET_VOLUME: u32 = 0xc010110a;
pub const SET_VOLUME: u32 = 0x40101109;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct VolumeDb {
    pub integer: i32,
    pub thousandths: i32,
}
#[repr(C)]
struct VolumeDbQuery {
    output: u32,
    path: u32,
    value: VolumeDb,
}
#[repr(C)]
struct Connections {
    output: u32,
    is_input: u32,
    module: u32,
    count: u32,
    array: u64,
}
#[repr(C)]
struct Connect {
    output: u32,
    input: u32,
    reserved: u8,
    pad: [u8; 3],
}
const _: () = {
    assert!(std::mem::size_of::<VolumeDbQuery>() == 16);
    assert!(std::mem::size_of::<Connections>() == 24);
    assert!(std::mem::offset_of!(Connections, array) == 16);
    assert!(std::mem::size_of::<Connect>() == 12);
};

/// The raw handle of the speaker output the stock HAL created (device 4).
pub fn speaker(transport: &impl Transport) -> io::Result<u32> {
    let mut wire = [4u32, 0];
    call(transport, GET_HANDLE, &mut wire)?;
    Ok(wire[1])
}
/// The raw number of inputs connected to `output`.
pub fn count(transport: &impl Transport, output: u32) -> io::Result<u32> {
    let mut wire = [output, 1u32, 0u32, 0u32];
    call(transport, GET_COUNT, &mut wire)?;
    Ok(wire[3])
}
/// The first `count` input handles connected to `output`.
pub fn connected(transport: &impl Transport, output: u32, count: u32) -> io::Result<Vec<u32>> {
    let mut handles = vec![0u32; count as usize];
    if count > 0 {
        let mut wire = Connections {
            output,
            is_input: 1,
            module: 0,
            count,
            array: handles.as_mut_ptr() as usize as u64,
        };
        call(transport, GET_CONNECTED, &mut wire)?;
    }
    Ok(handles)
}
pub fn connect(transport: &impl Transport, output: u32, input: u32) -> io::Result<()> {
    call(
        transport,
        CONNECT,
        &mut Connect {
            output,
            input,
            reserved: 0,
            pad: [0; 3],
        },
    )
}
pub fn disconnect(transport: &impl Transport, output: u32, input: u32) -> io::Result<()> {
    call(transport, DISCONNECT, &mut [output, input])
}
pub fn volume_db(transport: &impl Transport, output: u32) -> io::Result<VolumeDb> {
    let mut wire = VolumeDbQuery {
        output,
        path: 4,
        value: VolumeDb::default(),
    };
    call(transport, GET_VOLUME, &mut wire)?;
    Ok(wire.value)
}
pub fn set_volume_db(transport: &impl Transport, output: u32, value: VolumeDb) -> io::Result<()> {
    let mut wire = VolumeDbQuery {
        output,
        path: 4,
        value,
    };
    call(transport, SET_VOLUME, &mut wire)
}

#[cfg(test)]
#[path = "../../test/unit/audio/aout-route-tests.rs"]
mod tests;
