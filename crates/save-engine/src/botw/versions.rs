pub const VERSION: [&str; 16] = [
    "v1.0", "v1.1", "v1.2", "v1.3", "v1.3.1", "Kiosk", "v1.3.3", "v1.3.4", "v1.4", "v1.5", "v1.5*",
    "v1.6", "v1.6*", "v1.6**", "v1.6***", "v1.8",
];

pub const FILE_SIZE: [usize; 16] = [
    896976, 897160, 897112, 907824, 907824, 916576, 1020648, 1020648, 1027208, 1027208, 1027248,
    1027216, 1027216, 1027216, 1027216, 1027248,
];

pub const HEADER: [u32; 16] = [
    0x24e2, 0x24ee, 0x2588, 0x29c0, 0x2a46, 0x2f8e, 0x3ef8, 0x3ef9, 0x471a, 0x471b, 0x471b, 0x471e,
    0x0f423d, 0x0f423e, 0x0f423f, 0x4730,
];

#[derive(Debug, PartialEq, Eq)]
pub struct DetectedVersion {
    pub index: usize,
    pub little_endian: bool,
    pub modded: bool,
}

/// Mirrors `checkValidSavegame`/`_checkValidSavegameByConsole`: tries big-endian (Wii U)
/// then little-endian (Switch), matching file size + header against the known version
/// table, falling back to a header-only "modded" match for file sizes in the observed
/// modded range.
pub fn detect(bytes: &[u8]) -> Option<DetectedVersion> {
    for little_endian in [false, true] {
        if let Some(v) = detect_for_endianness(bytes, little_endian) {
            return Some(v);
        }
    }
    None
}

fn detect_for_endianness(bytes: &[u8], little_endian: bool) -> Option<DetectedVersion> {
    if bytes.len() < 8 {
        return None;
    }
    let header = read_u32_at(bytes, 0, little_endian);
    let footer = read_u32_at(bytes, 4, little_endian);
    if footer != 0xffffffff {
        return None;
    }

    for i in 0..VERSION.len() {
        if bytes.len() == FILE_SIZE[i] && header == HEADER[i] {
            return Some(DetectedVersion {
                index: i,
                little_endian,
                modded: false,
            });
        }
    }

    for i in 0..VERSION.len() {
        if (896976..=1500000).contains(&bytes.len()) && header == HEADER[i] {
            return Some(DetectedVersion {
                index: i,
                little_endian,
                modded: true,
            });
        }
    }

    None
}

fn read_u32_at(bytes: &[u8], offset: usize, little_endian: bool) -> u32 {
    let b = [
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ];
    if little_endian {
        u32::from_le_bytes(b)
    } else {
        u32::from_be_bytes(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_real_botw_fixture_as_v1_1_big_endian() {
        let bytes = std::fs::read("tests/fixtures/botw/game_data.sav").expect("fixture present");
        let detected = detect(&bytes).expect("should detect a known version");
        assert_eq!(detected.index, 1);
        assert_eq!(VERSION[detected.index], "v1.1");
        assert!(!detected.little_endian);
        assert!(!detected.modded);
    }

    #[test]
    fn rejects_too_short_input() {
        assert_eq!(detect(&[0u8; 4]), None);
    }
}
