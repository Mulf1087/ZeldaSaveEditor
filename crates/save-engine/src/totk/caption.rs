use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::hashtable;

/// `caption.sav` uses the same `(u32 hash, u32 value)` row shape as `progress.sav`'s hash
/// table (scanned from `0x28` by [`hashtable::scan_offsets`]), but the table has a small
/// fixed end instead of a sentinel-terminated one, mirroring `zelda-totk.js`'s
/// `captionReadU32ByHash`/`captionReadBoolByHash` loop bound.
const HASH_TABLE_END: usize = 0x1c0;

const HASHES: [(u32, &str, bool); 7] = [
    (0x9811a3f7, "YEAR", false),
    (0xdfd840d3, "MONTH", false),
    (0xbd46f485, "DAY", false),
    (0x23f3d75e, "HOUR", false),
    (0x27853bf7, "MINUTE", false),
    (0x25f03caa, "IS_AUTOSAVE", false),
    // The value stored at this hash is itself the address of the thumbnail blob
    // (a u32 size followed by raw JPEG bytes), the same one-level indirection
    // `scan_offsets`'s pointer hashes already model.
    (0x63696a32, "THUMBNAIL_OFFSET", true),
];

pub struct CaptionInfo {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub is_autosave: bool,
    pub thumbnail_jpeg: Vec<u8>,
}

/// Parses a `caption.sav` file: the save-slot thumbnail and the date/autosave metadata
/// shown next to it in the source's save-slot picker. Read-only, mirroring the source,
/// which never writes to this file.
pub fn parse(bytes: Vec<u8>) -> Result<CaptionInfo, SaveError> {
    let mut buf = SaveBuffer::new(bytes);
    buf.little_endian = true;

    let offsets = hashtable::scan_offsets(&buf, HASH_TABLE_END, &HASHES)?;

    let thumbnail_offset = offsets["THUMBNAIL_OFFSET"];
    let thumbnail_size = buf.read_u32(thumbnail_offset)? as usize;
    let thumbnail_jpeg = buf.read_bytes(thumbnail_offset + 4, thumbnail_size)?;

    Ok(CaptionInfo {
        year: buf.read_u32(offsets["YEAR"])?,
        month: buf.read_u32(offsets["MONTH"])?,
        day: buf.read_u32(offsets["DAY"])?,
        hour: buf.read_u32(offsets["HOUR"])?,
        minute: buf.read_u32(offsets["MINUTE"])?,
        is_autosave: buf.read_u8(offsets["IS_AUTOSAVE"])? != 0,
        thumbnail_jpeg,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_caption_sav() -> Vec<u8> {
        let mut data = vec![0u8; 0x28];
        let thumbnail_offset = HASH_TABLE_END as u32;
        let rows: [(u32, u32); 7] = [
            (0x9811a3f7, 2026),
            (0xdfd840d3, 7),
            (0xbd46f485, 30),
            (0x23f3d75e, 14),
            (0x27853bf7, 45),
            (0x25f03caa, 1),
            (0x63696a32, thumbnail_offset),
        ];
        for (hash, value) in rows {
            data.extend_from_slice(&hash.to_le_bytes());
            data.extend_from_slice(&value.to_le_bytes());
        }
        data.resize(HASH_TABLE_END, 0);

        let jpeg_bytes = [0xffu8, 0xd8, 0xff, 0xd9]; // SOI/EOI markers, contents don't matter to the parser
        data.extend_from_slice(&(jpeg_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(&jpeg_bytes);
        data
    }

    #[test]
    fn parses_date_autosave_flag_and_thumbnail() {
        let info = parse(synthetic_caption_sav()).unwrap();
        assert_eq!(info.year, 2026);
        assert_eq!(info.month, 7);
        assert_eq!(info.day, 30);
        assert_eq!(info.hour, 14);
        assert_eq!(info.minute, 45);
        assert!(info.is_autosave);
        assert_eq!(info.thumbnail_jpeg, vec![0xff, 0xd8, 0xff, 0xd9]);
    }

    #[test]
    fn missing_hash_row_is_an_error() {
        let mut data = vec![0u8; HASH_TABLE_END];
        // Only 6 of the 7 required hashes present.
        let mut i = 0x28;
        for (hash, value) in [
            (0x9811a3f7u32, 2026u32),
            (0xdfd840d3, 7),
            (0xbd46f485, 30),
            (0x23f3d75e, 14),
            (0x27853bf7, 45),
            (0x25f03caa, 0),
        ] {
            data[i..i + 4].copy_from_slice(&hash.to_le_bytes());
            data[i + 4..i + 8].copy_from_slice(&value.to_le_bytes());
            i += 8;
        }
        assert!(parse(data).is_err());
    }
}
