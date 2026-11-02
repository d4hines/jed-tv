#!/usr/bin/env python3
"""Run flightgpu's actual draw closure/calls against a host GL recorder."""
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "src/flightgpu/src/main.rs"
OUT = ROOT / "assets/build/gpu-path-check"


def main():
    source = SOURCE.read_text()
    closure = re.search(r"    let draw = .*?\n    };", source, re.S).group(0)
    calls = re.findall(r"^\s+(draw\(vbo\[.*?;)$", source, re.M)
    if len(calls) != 4:
        raise SystemExit(f"Expected four geometry draw calls, found {len(calls)}")
    harness = r'''
#![allow(non_snake_case)]
use std::cell::RefCell;
use std::ffi::c_void;
thread_local! {
    static BOUND: RefCell<u32> = const { RefCell::new(0) };
    static CALLS: RefCell<Vec<(u32,u32,i32,i32)>> = const { RefCell::new(Vec::new()) };
}
const GL_ARRAY_BUFFER: u32 = 0x8892;
const GL_FLOAT: u32 = 0x1406;
const GL_TRIANGLES: u32 = 4;
const GL_LINES: u32 = 1;
struct Gl {
    BindBuffer: unsafe extern "C" fn(u32,u32),
    VertexAttribPointer: unsafe extern "C" fn(u32,i32,u32,u8,i32,*const c_void),
    EnableVertexAttribArray: unsafe extern "C" fn(u32),
    DrawArrays: unsafe extern "C" fn(u32,i32,i32),
}
unsafe extern "C" fn bind(target:u32, buffer:u32) {
    assert_eq!(target, GL_ARRAY_BUFFER);
    BOUND.with(|b| *b.borrow_mut() = buffer);
}
unsafe extern "C" fn attrib(index:u32, size:i32, kind:u32, normalized:u8, stride:i32, ptr:*const c_void) {
    assert_eq!((size,kind,normalized,stride),(3,GL_FLOAT,0,24));
    assert_eq!(ptr as usize, if index == 0 {0} else {12});
}
unsafe extern "C" fn enable(_:u32) {}
unsafe extern "C" fn draw_arrays(mode:u32, first:i32, count:i32) {
    BOUND.with(|b| CALLS.with(|c| c.borrow_mut().push((*b.borrow(),mode,first,count))));
}
fn main() {
    let gl = &Gl { BindBuffer:bind, VertexAttribPointer:attrib,
                  EnableVertexAttribArray:enable, DrawArrays:draw_arrays };
    let (p_attr,c_attr)=(0,1);
    let vbo = [11,12,13];
    let terrain = vec![0f32; 36];
    let boxes = vec![0f32; 54];
    /* DRAW_CLOSURE */
    for aircraft_verts in [0i32, 3, 18, 72] {
        for path_vertices in [0i32, 2, 4, 510] {
            // The aircraft prefix is deliberately distinct from every path vertex.
            let mut dynamic = vec![10f32; aircraft_verts as usize * 6];
            dynamic.extend((0..path_vertices).flat_map(|i| [1000.0+i as f32;6]));
            CALLS.with(|c| c.borrow_mut().clear());
            /* DRAW_CALLS */
            CALLS.with(|c| {
                let calls=c.borrow();
                assert_eq!(&calls[..3], &[(12,GL_TRIANGLES,0,6),(11,GL_TRIANGLES,0,9),
                                         (13,GL_TRIANGLES,0,aircraft_verts)]);
                let &(buffer,mode,first,count)=&calls[3];
                assert_eq!((buffer,mode),(13,GL_LINES));
                assert_eq!(count,path_vertices);
                let selected=&dynamic[first as usize*6..(first+count) as usize*6];
                let expected:Vec<f32>=(0..path_vertices).flat_map(|i| [1000.0+i as f32;6]).collect();
                assert_eq!(selected,expected,"escape draw selected aircraft or skipped path vertices");
            });
        }
    }
    println!("PASS: 16 geometry layouts; exact production GL calls select only the full escape path");
}
'''
    harness = harness.replace("/* DRAW_CLOSURE */", closure).replace("/* DRAW_CALLS */", "\n".join(calls))
    OUT.mkdir(parents=True, exist_ok=True)
    generated = OUT / "check.rs"
    generated.write_text(harness)
    executable = OUT / "check"
    subprocess.run(["rustc", "+1.92.0", "--edition=2021", str(generated), "-o", str(executable)], check=True)
    subprocess.run([str(executable)], check=True)


if __name__ == "__main__":
    main()
