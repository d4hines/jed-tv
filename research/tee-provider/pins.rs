use g06_tee_rpc::{DirectoryStore, LoadError, TaStore, Uuid, AUDIO_TA};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io, path::Path};
pub const ROOT: &str = "/mnt/vendor/tvservice/glibc/optee_armtz";
#[derive(Clone, Copy)]
pub struct Pin {
    pub uuid: Uuid,
    pub bytes: usize,
    pub sha256: &'static str,
}
pub const PINS: [Pin; 4] = [
    Pin {
        uuid: Uuid([
            0x1f, 0xc1, 0x70, 0x90, 0xbb, 0xdf, 0x46, 0x48, 0xbc, 0xa9, 0x3a, 0xf9, 0x7a, 0xd8,
            0x8e, 0x63,
        ]),
        bytes: 149600,
        sha256: "fae08300ca9157395f7fbe98c1666314f188966685c5a4ad437ebf23ab6900f5",
    },
    Pin {
        uuid: AUDIO_TA,
        bytes: 145328,
        sha256: "ee8fd2adf80cb81a96643c4198fe06c862fa65beaf1ea4c6966648da31e430a2",
    },
    Pin {
        uuid: Uuid([
            0x09, 0x87, 0xbe, 0x40, 0x05, 0xfb, 0x11, 0xe6, 0x98, 0x93, 0x00, 0x02, 0xa5, 0xd5,
            0xc5, 0x1b,
        ]),
        bytes: 484192,
        sha256: "48f2971ce5f8c7778ad42db655101ba4b91fb5c08f7bc1f72c3d9daaad69144b",
    },
    Pin {
        uuid: Uuid([
            0x99, 0x98, 0x60, 0x25, 0x2c, 0x8d, 0x43, 0xca, 0x95, 0x98, 0xb9, 0x0e, 0x32, 0x4e,
            0x9d, 0x3b,
        ]),
        bytes: 190528,
        sha256: "55376a0a3bfaa1223399b4b86f88ee6c8053db4b29fdaa05cd1f1ccf8a8958e4",
    },
];
fn verify(pin: Pin, data: &[u8]) -> bool {
    data.len() == pin.bytes && format!("{:x}", Sha256::digest(data)) == pin.sha256
}
pub struct PinnedStore {
    files: BTreeMap<Uuid, Vec<u8>>,
}
impl PinnedStore {
    /// All known files are verified and cached before the TEE device opens.
    pub fn preload(root: &Path) -> io::Result<Self> {
        let mut loader = DirectoryStore::new(
            root.to_path_buf(),
            PINS.iter().map(|p| (p.uuid, p.bytes)).collect(),
        );
        let mut files = BTreeMap::new();
        for pin in PINS {
            let data = loader
                .load(pin.uuid, 1024 * 1024)
                .map_err(|e| io::Error::other(format!("TA preload {} failed: {e:?}", pin.uuid)))?;
            if !verify(pin, &data) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("TA pin mismatch: {}", pin.uuid),
                ));
            }
            files.insert(pin.uuid, data);
        }
        Ok(Self { files })
    }
}
impl TaStore for PinnedStore {
    fn load(&mut self, uuid: Uuid, max_bytes: usize) -> Result<Vec<u8>, LoadError> {
        let data = self.files.get(&uuid).ok_or(LoadError::NotFound)?;
        if data.len() > max_bytes {
            return Err(LoadError::TooLarge);
        }
        Ok(data.clone())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pins_are_unique_bounded_and_well_formed() {
        let ids: BTreeMap<_, _> = PINS.iter().map(|p| (p.uuid, p.bytes)).collect();
        assert_eq!(ids.len(), 4);
        for p in PINS {
            assert!(p.bytes < 1024 * 1024);
            assert_eq!(p.sha256.len(), 64);
            assert!(p.sha256.bytes().all(|c| c.is_ascii_hexdigit()));
        }
    }
    #[test]
    fn digest_and_length_must_both_match() {
        let pin = Pin {
            uuid: AUDIO_TA,
            bytes: 3,
            sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        };
        assert!(verify(pin, b"abc"));
        assert!(!verify(pin, b"abd"));
        assert!(!verify(pin, b"abcd"));
    }
    #[test]
    fn missing_uuid_fails_instead_of_directory_search() {
        let mut store = PinnedStore {
            files: BTreeMap::new(),
        };
        assert!(matches!(
            store.load(Uuid([0; 16]), 1024),
            Err(LoadError::NotFound)
        ));
    }
}
