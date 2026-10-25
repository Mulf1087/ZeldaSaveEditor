use crate::error::SaveError;

/// (version label, file size, header value at offset 4, metaDataStart value at offset 8).
/// Used only to LABEL a detected save. An unrecognized combination is labeled "game mod"
/// rather than rejected; the real validity gate is `check_magic_and_size` plus the
/// hash-table scan in `totk::hashtable`.
pub const GAME_VERSIONS: [(&str, usize, u32, u32); 3] = [
    ("v1.0", 2307552, 0x0046c3c8, 0x0003c050),
    ("v1.1.x/v1.2.x", 2307656, 0x0047e0f4, 0x0003c088),
    ("v1.4.x", 2307856, 0x0049e946, 0x0003c138),
];

const MIN_FILE_SIZE: usize = 2307552;
const MAX_FILE_SIZE_EXCLUSIVE: usize = 4194304;

/// Mirrors the magic + file-size range check in `checkValidSavegame`, before any
/// hash-table scanning happens. TOTK saves are always little-endian.
pub fn check_magic_and_size(bytes: &[u8]) -> Result<(), SaveError> {
    if bytes.len() < 12 {
        return Err(SaveError::UnknownFormat);
    }
    let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if magic == 0x01020304 && bytes.len() >= MIN_FILE_SIZE && bytes.len() < MAX_FILE_SIZE_EXCLUSIVE {
        Ok(())
    } else {
        Err(SaveError::UnknownFormat)
    }
}

/// Labels the save's version by matching file size + header (u32 at offset 4) +
/// metaDataStart (u32 at offset 8) against `GAME_VERSIONS`. Returns `(label, modded)`.
/// Only call this after `check_magic_and_size` has succeeded (it relies on the buffer
/// being at least 12 bytes long, which that check already guarantees given
/// `MIN_FILE_SIZE` is far larger than 12).
pub fn label(bytes: &[u8]) -> (&'static str, bool) {
    let header = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let meta_data_start = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    for &(name, file_size, expected_header, expected_meta) in GAME_VERSIONS.iter() {
        if bytes.len() == file_size && header == expected_header && meta_data_start == expected_meta {
            return (name, false);
        }
    }
    ("game mod", true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_real_totk_fixture() {
        let bytes = std::fs::read("tests/fixtures/totk/progress.sav").expect("fixture present");
        assert!(check_magic_and_size(&bytes).is_ok());
    }

    #[test]
    fn labels_real_fixture_as_v1_1_x_v1_2_x() {
        let bytes = std::fs::read("tests/fixtures/totk/progress.sav").expect("fixture present");
        let (label_str, modded) = label(&bytes);
        assert_eq!(label_str, "v1.1.x/v1.2.x");
        assert!(!modded);
    }

    #[test]
    fn rejects_wrong_magic() {
        let mut bytes = vec![0u8; MIN_FILE_SIZE];
        bytes[0..4].copy_from_slice(&0xffffffffu32.to_le_bytes());
        assert_eq!(check_magic_and_size(&bytes), Err(SaveError::UnknownFormat));
    }

    #[test]
    fn rejects_too_short_input() {
        assert_eq!(check_magic_and_size(&[1, 2, 3, 4]), Err(SaveError::UnknownFormat));
    }
}
