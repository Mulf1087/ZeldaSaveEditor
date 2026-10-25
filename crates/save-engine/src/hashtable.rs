use std::collections::HashMap;

use crate::binary::SaveBuffer;

/// Scans `buf` once for each `(hash, name)` pair in `hashes`, in order, and records the
/// byte offset immediately after each match (where that property's value lives).
/// `hashes` must be sorted in ascending hash order matching the file's own layout,
/// since the search cursor only ever moves forward.
pub fn scan_offsets(buf: &SaveBuffer, hashes: &[(u32, &'static str)]) -> HashMap<&'static str, usize> {
    let mut offsets = HashMap::new();
    let mut start = 0x0c;
    for &(hash, name) in hashes {
        let mut j = start;
        while j + 8 <= buf.len() {
            if buf.read_u32(j).expect("bounds checked by the loop guard immediately above") == hash {
                offsets.insert(name, j + 4);
                start = j + 8;
                break;
            }
            j += 8;
        }
    }
    offsets
}

/// Builds a hash -> value-offset map for *every* entry in the hash table, not just a known
/// subset, mirroring `_searchHash`'s "scan the whole table from the start" lookup used for
/// hashes outside the small fixed `Hashes` list (completionism flags: koroks, defeated
/// hinox/talus/molduga, locations). Unlike `scan_offsets`, entries don't need to be sorted or
/// requested up front: build once, then look up as many arbitrary hashes as needed.
///
/// First occurrence wins when a hash repeats, matching `_searchHash` always returning the
/// first match found scanning forward from the start of the table.
pub fn build_full_index(buf: &SaveBuffer) -> HashMap<u32, usize> {
    let mut index = HashMap::new();
    let mut j = 0x0c;
    while j + 8 <= buf.len() {
        let hash = buf.read_u32(j).expect("bounds checked by the loop guard immediately above");
        index.entry(hash).or_insert(j + 4);
        j += 8;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer_with_slots(slots: &[(u32, u32)]) -> SaveBuffer {
        let mut data = vec![0u8; 0x0c];
        for (hash, value) in slots {
            data.extend_from_slice(&hash.to_be_bytes());
            data.extend_from_slice(&value.to_be_bytes());
        }
        SaveBuffer::new(data)
    }

    #[test]
    fn finds_offsets_for_known_hashes_in_order() {
        let buf = buffer_with_slots(&[(0x1111, 10), (0x2222, 20), (0x3333, 30)]);
        let offsets = scan_offsets(&buf, &[(0x1111, "A"), (0x2222, "B"), (0x3333, "C")]);
        assert_eq!(offsets.len(), 3);
        assert_eq!(buf.read_u32(offsets["A"]).unwrap(), 10);
        assert_eq!(buf.read_u32(offsets["B"]).unwrap(), 20);
        assert_eq!(buf.read_u32(offsets["C"]).unwrap(), 30);
    }

    #[test]
    fn build_full_index_resolves_arbitrary_hashes_not_in_a_known_list() {
        let buf = buffer_with_slots(&[(0x1111, 10), (0x2222, 20), (0x3333, 30)]);
        let index = build_full_index(&buf);
        assert_eq!(index.len(), 3);
        assert_eq!(buf.read_u32(index[&0x2222]).unwrap(), 20);
        assert!(!index.contains_key(&0x9999));
    }

    #[test]
    fn build_full_index_keeps_first_occurrence_of_a_repeated_hash() {
        let buf = buffer_with_slots(&[(0x1111, 10), (0x1111, 99)]);
        let index = build_full_index(&buf);
        assert_eq!(buf.read_u32(index[&0x1111]).unwrap(), 10);
    }

    #[test]
    fn missing_hash_is_absent_but_does_not_block_later_ones() {
        let buf = buffer_with_slots(&[(0x1111, 10), (0x3333, 30)]);
        let offsets = scan_offsets(&buf, &[(0x1111, "A"), (0x2222, "B"), (0x3333, "C")]);
        assert_eq!(offsets.len(), 2);
        assert!(!offsets.contains_key("B"));
        assert_eq!(buf.read_u32(offsets["C"]).unwrap(), 30);
    }
}
