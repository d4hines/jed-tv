//! Exact V109 read-only vendor object RPCs, with checked owned-SHM access.
use super::*;

const HEADER_BYTES: usize = 28;

/// The object ABI embeds a native ARM32 TEEC_UUID, unlike LOAD_TA value octets.
pub(super) fn header_uuid(header: &[u8; HEADER_BYTES]) -> Uuid {
    let mut bytes: [u8; 16] = header[..16].try_into().unwrap();
    bytes[..4].reverse();
    bytes[4..6].reverse();
    bytes[6..8].reverse();
    Uuid(bytes)
}

impl<A: Allocator, S: TaStore> Provider<A, S> {
    fn object_range(&self, p: &Param) -> Option<(i32, usize, usize)> {
        let id = i32::try_from(p.c).ok()?;
        let owned = self.regions.get(&id)?;
        // Check the full 64-bit wire range before converting to host usize.
        let end = p.a.checked_add(p.b)?;
        if end > owned.requested as u64 {
            return None;
        }
        Some((id, usize::try_from(p.a).ok()?, usize::try_from(p.b).ok()?))
    }

    pub(super) fn vendor_object(&mut self, function: u32, params: &mut [Param]) -> u32 {
        let reading = function == LOAD_OBJECT;
        if params.len() != if reading { 2 } else { 1 } || params[0].attr != 7 {
            return BAD_PARAMETERS;
        }
        let Some((header_id, header_offset, header_capacity)) = self.object_range(&params[0])
        else {
            return BAD_PARAMETERS;
        };
        if header_capacity < HEADER_BYTES {
            return BAD_PARAMETERS;
        }
        let destination = if reading {
            if params[1].attr != 6 && params[1].attr != 7 {
                return BAD_PARAMETERS;
            }
            let Some((id, offset, capacity)) = self.object_range(&params[1]) else {
                return BAD_PARAMETERS;
            };
            // A read must not overwrite its own result header, including UUID.
            if id == header_id
                && capacity != 0
                && offset < header_offset + HEADER_BYTES
                && header_offset < offset + capacity
            {
                return BAD_PARAMETERS;
            }
            Some((id, offset, capacity))
        } else {
            None
        };
        let mut header = [0; HEADER_BYTES];
        fence(Ordering::Acquire);
        if self
            .regions
            .get(&header_id)
            .unwrap()
            .region
            .read(header_offset, &mut header)
            .is_err()
        {
            return GENERIC;
        }
        let file_offset = u32::from_le_bytes(header[16..20].try_into().unwrap());
        if reading && file_offset > i32::MAX as u32 {
            return BAD_PARAMETERS;
        }
        let limit = self.limits.ta_bytes.min(i32::MAX as usize);
        let data = match self.store.load_object(header_uuid(&header), limit) {
            Ok(data) if data.len() <= limit => data,
            Ok(_) | Err(LoadError::TooLarge) => return OUT_OF_MEMORY,
            Err(_) => return if reading { ITEM_NOT_FOUND } else { GENERIC },
        };
        let (field, result) = if let Some((id, offset, capacity)) = destination {
            let start = (file_offset as usize).min(data.len());
            let count = capacity.min(data.len() - start);
            if count != 0
                && self
                    .regions
                    .get_mut(&id)
                    .unwrap()
                    .region
                    .write(offset, &data[start..start + count])
                    .is_err()
            {
                return GENERIC;
            }
            (24, count as u32)
        } else {
            (20, data.len() as u32)
        };
        if self
            .regions
            .get_mut(&header_id)
            .unwrap()
            .region
            .write(header_offset + field, &result.to_le_bytes())
            .is_err()
        {
            return GENERIC;
        }
        fence(Ordering::Release);
        SUCCESS
    }
}
