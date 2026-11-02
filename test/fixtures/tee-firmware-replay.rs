//! Offline packet peer for comparisons with the extracted stock ARM handler.
//! The only filesystem input is the pinned TA directory. No TEE device opens.
use g06_tee_provider::{pins::PinnedStore, serve_one, Packet, Transport};
use g06_tee_rpc::{Allocator, Limits, Param, Provider, Region};
use sha2::{Digest, Sha256};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::{self, BufRead},
    rc::Rc,
};

type Memory = Rc<RefCell<BTreeMap<i32, Vec<u8>>>>;
struct Heap {
    memory: Memory,
    next: i32,
}
struct Buffer {
    memory: Memory,
    id: i32,
    bytes: usize,
}
impl Region for Buffer {
    fn id(&self) -> i32 {
        self.id
    }
    fn len(&self) -> usize {
        self.bytes
    }
    fn clear(&mut self) -> io::Result<()> {
        self.memory.borrow_mut().get_mut(&self.id).unwrap().fill(0);
        Ok(())
    }
    fn read(&self, offset: usize, data: &mut [u8]) -> io::Result<()> {
        let memory = self.memory.borrow();
        let source = memory
            .get(&self.id)
            .unwrap()
            .get(
                offset
                    ..offset
                        .checked_add(data.len())
                        .ok_or_else(|| io::Error::other("overflow"))?,
            )
            .ok_or_else(|| io::Error::other("bounds"))?;
        data.copy_from_slice(source);
        Ok(())
    }
    fn write(&mut self, offset: usize, data: &[u8]) -> io::Result<()> {
        let mut memory = self.memory.borrow_mut();
        let target = memory
            .get_mut(&self.id)
            .unwrap()
            .get_mut(
                offset
                    ..offset
                        .checked_add(data.len())
                        .ok_or_else(|| io::Error::other("overflow"))?,
            )
            .ok_or_else(|| io::Error::other("bounds"))?;
        target.copy_from_slice(data);
        Ok(())
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        self.memory.borrow_mut().remove(&self.id);
    }
}
impl Allocator for Heap {
    type Region = Buffer;
    fn allocate(&mut self, bytes: usize) -> io::Result<Buffer> {
        let id = self.next;
        self.next += 1;
        self.memory.borrow_mut().insert(id, vec![0; bytes]);
        Ok(Buffer {
            memory: self.memory.clone(),
            id,
            bytes,
        })
    }
}
struct Wire {
    input: Packet,
    output: Option<Packet>,
}
impl Transport for Wire {
    fn receive(&mut self, p: &mut Packet) -> io::Result<()> {
        *p = self.input.clone();
        Ok(())
    }
    fn send(&mut self, p: &Packet) -> io::Result<()> {
        self.output = Some(p.clone());
        Ok(())
    }
}
fn decode(text: &str) -> Vec<u8> {
    assert_eq!(text.len() % 2, 0);
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex packet"))
        .collect()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() -> io::Result<()> {
    let root = std::env::args().nth(1).expect("pinned TA root");
    let store = PinnedStore::preload(std::path::Path::new(&root))?;
    let memory = Memory::default();
    let mut provider = Provider::new(
        Heap {
            memory: memory.clone(),
            next: 17,
        },
        store,
        Limits::default(),
    )
    .unwrap();
    for line in io::stdin().lock().lines() {
        let raw = decode(&line?);
        assert!(raw.len() >= 8);
        let count = u32::from_le_bytes(raw[4..8].try_into().unwrap());
        assert!(count <= 31 && raw.len() == 8 + 32 * count as usize);
        let mut input = Packet::receive_buffer();
        input.word = u32::from_le_bytes(raw[..4].try_into().unwrap());
        input.count = count;
        for (p, bytes) in input.params.iter_mut().zip(raw[8..].chunks_exact(32)) {
            let mut words = bytes
                .chunks_exact(8)
                .map(|b| u64::from_le_bytes(b.try_into().unwrap()));
            *p = Param {
                attr: words.next().unwrap(),
                a: words.next().unwrap(),
                b: words.next().unwrap(),
                c: words.next().unwrap(),
            };
        }
        let mut wire = Wire {
            input,
            output: None,
        };
        serve_one(&mut wire, &mut provider)?;
        let output = wire.output.unwrap();
        let mut bytes = Vec::from(output.word.to_le_bytes());
        bytes.extend(output.count.to_le_bytes());
        for p in &output.params[..output.count as usize] {
            for v in [p.attr, p.a, p.b, p.c] {
                bytes.extend(v.to_le_bytes());
            }
        }
        let hashes = memory
            .borrow()
            .iter()
            .map(|(id, data)| format!("{id}:{:x}", Sha256::digest(data)))
            .collect::<Vec<_>>()
            .join(",");
        println!("{};{hashes}", hex(&bytes));
    }
    Ok(())
}
