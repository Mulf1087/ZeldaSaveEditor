use crate::binary::SaveBuffer;
use crate::error::SaveError;

pub const STRING64_SIZE: usize = 0x80;

/// Reads `groups` groups of up to 4 ASCII characters, 8 bytes apart, mirroring
/// `_readString`. Each group stops early at a null byte but does not affect later
/// groups, matching the upstream behavior exactly.
pub fn read_padded_string(buf: &SaveBuffer, offset: usize, groups: usize) -> Result<String, SaveError> {
    let mut out = String::new();
    for i in 0..groups {
        out.push_str(&buf.read_string(offset + i * 8, 4));
    }
    Ok(out)
}

/// Writes `value` across `groups` groups of 4 bytes, 8 bytes apart, zero-padding any
/// unused bytes in the final group, mirroring `_writeString`.
///
/// Assumes ASCII item/location identifiers, true for every known BOTW/TOTK save string.
/// A non-ASCII byte read via `read_padded_string` would not round-trip byte-exact if
/// written back through this function.
pub fn write_padded_string(buf: &mut SaveBuffer, offset: usize, value: &str, groups: usize) -> Result<(), SaveError> {
    let bytes = value.as_bytes();
    for i in 0..groups {
        let group_offset = offset + i * 8;
        for j in 0..4 {
            let idx = i * 4 + j;
            buf.write_u8(group_offset + j, bytes.get(idx).copied().unwrap_or(0))?;
        }
    }
    Ok(())
}

pub fn read_string64(buf: &SaveBuffer, offset: usize, array_index: usize) -> Result<String, SaveError> {
    read_padded_string(buf, offset + STRING64_SIZE * array_index, 16)
}

pub fn write_string64(buf: &mut SaveBuffer, offset: usize, array_index: usize, value: &str) -> Result<(), SaveError> {
    write_padded_string(buf, offset + STRING64_SIZE * array_index, value, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_short_string() {
        let mut buf = SaveBuffer::new(vec![0u8; 64]);
        write_padded_string(&mut buf, 0, "MainField", 8).unwrap();
        assert_eq!(read_padded_string(&buf, 0, 8).unwrap(), "MainField");
    }

    #[test]
    fn string64_uses_array_index_stride() {
        let mut buf = SaveBuffer::new(vec![0u8; STRING64_SIZE * 3]);
        write_string64(&mut buf, 0, 1, "Weapon_Sword_070").unwrap();
        assert_eq!(read_string64(&buf, 0, 1).unwrap(), "Weapon_Sword_070");
        assert_eq!(read_string64(&buf, 0, 0).unwrap(), "");
        assert_eq!(read_string64(&buf, 0, 2).unwrap(), "");
    }

    #[test]
    fn write_pads_remaining_bytes_with_zero() {
        let mut buf = SaveBuffer::new(vec![0xffu8; 16]);
        write_padded_string(&mut buf, 0, "Hi", 2).unwrap();
        assert_eq!(read_padded_string(&buf, 0, 2).unwrap(), "Hi");
        assert_eq!(buf.read_u8(2).unwrap(), 0);
        assert_eq!(buf.read_u8(3).unwrap(), 0);
    }

    #[test]
    fn write_padded_string_on_too_short_buffer_returns_err_instead_of_panicking() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        // A single 4-byte group starting at offset 0 needs bytes 0..=3 for the group
        // itself, but write_padded_string always writes exactly 4 bytes per group
        // regardless of `value`'s length, so a second group has no room at all.
        let result = write_padded_string(&mut buf, 0, "Hi", 2);
        assert!(matches!(result, Err(SaveError::Truncated { .. })));
    }
}
