use crate::binary::SaveBuffer;
use crate::error::SaveError;

pub const STRING64_SIZE: usize = 0x40;

/// Reads a null-terminated ASCII string from a single fixed 0x40-byte block, TOTK's
/// `String64` type. Simpler than BOTW's 8-byte-strided padded strings: no interleaved
/// padding here, just one contiguous block.
pub fn read_string64(buf: &SaveBuffer, offset: usize) -> String {
    buf.read_string(offset, STRING64_SIZE)
}

/// Writes `value`'s bytes into a 0x40-byte block starting at `offset`, zero-padding any
/// remaining bytes. Mirrors `tempFile.writeString(offset, value, 0x40)`.
///
/// Assumes ASCII strings, true for TOTK's checkpoint/location identifiers and the same
/// assumption BOTW's padded-string helpers make.
pub fn write_string64(buf: &mut SaveBuffer, offset: usize, value: &str) -> Result<(), SaveError> {
    let bytes = value.as_bytes();
    for i in 0..STRING64_SIZE {
        buf.write_u8(offset + i, bytes.get(i).copied().unwrap_or(0))?;
    }
    Ok(())
}

pub const WSTRING16_SIZE: usize = 0x20;

/// Reads a null-terminated UTF-16LE string from a fixed 0x20-byte block, TOTK's
/// `WString16` type (used for horse names). Mirrors `Variable._read`'s `WString16` branch:
/// reads u16 code units until a zero code unit or the block ends.
///
/// Treats each u16 as a raw `char` via `char::from_u32`, skipping any code unit that isn't
/// valid on its own; surrogate pairs are not reassembled. Real TOTK horse names are short
/// ASCII/Latin strings (source enforces `maxLength: 9`), so this limitation is unlikely to
/// matter in practice. Revisit if a real save turns up a name using characters outside the BMP.
pub fn read_wstring16(buf: &SaveBuffer, offset: usize) -> Result<String, SaveError> {
    let mut out = String::new();
    for i in (0..WSTRING16_SIZE).step_by(2) {
        let lo = buf.read_u8(offset + i)?;
        let hi = buf.read_u8(offset + i + 1)?;
        let code = u16::from_le_bytes([lo, hi]);
        if code == 0 {
            break;
        }
        if let Some(c) = char::from_u32(code as u32) {
            out.push(c);
        }
    }
    Ok(out)
}

/// Writes `value` as UTF-16LE into a 0x20-byte block, zero-padding the remainder. Mirrors
/// `Variable._save`'s `WString16` branch.
pub fn write_wstring16(buf: &mut SaveBuffer, offset: usize, value: &str) -> Result<(), SaveError> {
    let mut units: Vec<u16> = value.encode_utf16().collect();
    units.truncate(WSTRING16_SIZE / 2);
    for i in 0..(WSTRING16_SIZE / 2) {
        let code = units.get(i).copied().unwrap_or(0);
        let bytes = code.to_le_bytes();
        buf.write_u8(offset + i * 2, bytes[0])?;
        buf.write_u8(offset + i * 2 + 1, bytes[1])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_short_string() {
        let mut buf = SaveBuffer::new(vec![0xffu8; STRING64_SIZE]);
        write_string64(&mut buf, 0, "MainField").unwrap();
        assert_eq!(read_string64(&buf, 0), "MainField");
    }

    #[test]
    fn write_pads_remaining_bytes_with_zero() {
        let mut buf = SaveBuffer::new(vec![0xffu8; STRING64_SIZE]);
        write_string64(&mut buf, 0, "Hi").unwrap();
        assert_eq!(buf.read_u8(2).unwrap(), 0);
        assert_eq!(buf.read_u8(STRING64_SIZE - 1).unwrap(), 0);
    }

    #[test]
    fn write_out_of_bounds_returns_err_instead_of_panicking() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        assert!(write_string64(&mut buf, 0, "too long for this four-byte buffer").is_err());
    }
}

#[cfg(test)]
mod wstring16_tests {
    use super::*;

    #[test]
    fn round_trips_a_short_name() {
        let mut buf = SaveBuffer::new(vec![0xffu8; WSTRING16_SIZE]);
        write_wstring16(&mut buf, 0, "Max").unwrap();
        assert_eq!(read_wstring16(&buf, 0).unwrap(), "Max");
    }

    #[test]
    fn round_trips_the_real_fixtures_longest_observed_horse_name() {
        // "Brownie" is horse index 1's real name in progress.sav (OwnedHorseList.Name).
        // Exercises the round-trip contract with a realistic name length rather than a
        // short placeholder string.
        let mut buf = SaveBuffer::new(vec![0u8; WSTRING16_SIZE]);
        write_wstring16(&mut buf, 0, "Brownie").unwrap();
        assert_eq!(read_wstring16(&buf, 0).unwrap(), "Brownie");
    }

    #[test]
    fn write_pads_remaining_bytes_with_zero() {
        let mut buf = SaveBuffer::new(vec![0xffu8; WSTRING16_SIZE]);
        write_wstring16(&mut buf, 0, "Hi").unwrap();
        assert_eq!(buf.read_u8(4).unwrap(), 0);
        assert_eq!(buf.read_u8(WSTRING16_SIZE - 1).unwrap(), 0);
    }
}
