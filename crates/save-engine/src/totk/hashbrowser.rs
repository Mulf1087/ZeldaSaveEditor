use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::hashdict;
use crate::totk::hashtable::HASH_TABLE_START;

/// A single row of the flat hash table, resolved against the vendored dictionary where known.
/// Scalar fields (`Bool`/`Int`/`UInt`/`Enum`/`Float`, stored inline) carry a decoded, editable
/// `value`; pointer fields (arrays, strings, vectors) carry only their raw stored address, since
/// their true length lives in a separate count/capacity field this crate doesn't generically
/// know, editing them here could silently corrupt whatever blob-heap data follows.
pub struct HashRow {
    pub hash: u32,
    pub name: Option<&'static str>,
    pub type_name: &'static str,
    pub editable: bool,
    pub value: f64,
}

fn decode_scalar(buf: &SaveBuffer, offset: usize, type_name: &str) -> Result<f64, SaveError> {
    Ok(match type_name {
        "Bool" => {
            if buf.read_u8(offset)? != 0 {
                1.0
            } else {
                0.0
            }
        }
        "Int" => buf.read_i32(offset)? as f64,
        "Float" => buf.read_f32(offset)? as f64,
        _ => buf.read_u32(offset)? as f64, // UInt, Enum
    })
}

fn encode_scalar(buf: &mut SaveBuffer, offset: usize, type_name: &str, value: f64) -> Result<(), SaveError> {
    match type_name {
        "Bool" => buf.write_u8(offset, if value != 0.0 { 1 } else { 0 }),
        "Int" => buf.write_i32(offset, value as i32),
        "Float" => buf.write_f32(offset, value as f32),
        _ => buf.write_u32(offset, value as u32), // UInt, Enum
    }
}

/// Lists every hash actually present in this save's flat table (`0x28..hash_table_end`).
pub fn browse(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<HashRow>, SaveError> {
    let mut rows = Vec::new();
    let mut i = HASH_TABLE_START;
    while i + 8 <= hash_table_end {
        let hash = buf.read_u32(i)?;
        let raw_value = buf.read_u32(i + 4)?;
        let row = match hashdict::lookup(hash) {
            Some(entry) if !entry.is_pointer => HashRow {
                hash,
                name: Some(entry.name),
                type_name: entry.type_name,
                editable: true,
                value: decode_scalar(buf, i + 4, entry.type_name)?,
            },
            Some(entry) => HashRow {
                hash,
                name: Some(entry.name),
                type_name: entry.type_name,
                editable: false,
                value: raw_value as f64,
            },
            None => HashRow { hash, name: None, type_name: "Unknown", editable: false, value: raw_value as f64 },
        };
        rows.push(row);
        i += 8;
    }
    Ok(rows)
}

/// Writes a new value to a scalar (non-pointer, dictionary-known) field found by `hash`,
/// re-scanning the table for the matching row rather than trusting a caller-supplied offset,
/// so a write always lands on a row that's actually there.
pub fn write_scalar(buf: &mut SaveBuffer, hash_table_end: usize, hash: u32, value: f64) -> Result<(), SaveError> {
    let entry = hashdict::lookup(hash).filter(|e| !e.is_pointer).ok_or(SaveError::MissingField("hash field"))?;
    let type_name = entry.type_name;

    let mut i = HASH_TABLE_START;
    while i + 8 <= hash_table_end {
        if buf.read_u32(i)? == hash {
            return encode_scalar(buf, i + 4, type_name, value);
        }
        i += 8;
    }
    Err(SaveError::MissingField("hash field"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::totk::murmur3::hash32;

    fn buffer_with_slots(slots: &[(u32, u32)]) -> SaveBuffer {
        let mut data = vec![0u8; HASH_TABLE_START];
        for (hash, value) in slots {
            data.extend_from_slice(&hash.to_le_bytes());
            data.extend_from_slice(&value.to_le_bytes());
        }
        let mut buf = SaveBuffer::new(data);
        buf.little_endian = true;
        buf
    }

    #[test]
    fn browse_decodes_known_scalar_and_flags_pointer_rows_as_not_editable() {
        let direct_hash = hash32("PlayerStatus.MaxLife"); // Int, direct
        let pointer_hash = hash32("Pouch.Weapon.ValidNum"); // IntArray, pointer
        let buf = buffer_with_slots(&[(direct_hash, 100), (pointer_hash, 0xabcd)]);
        let hash_table_end = buf.len();

        let rows = browse(&buf, hash_table_end).unwrap();
        assert_eq!(rows.len(), 2);

        let direct = rows.iter().find(|r| r.hash == direct_hash).unwrap();
        assert!(direct.editable);
        assert_eq!(direct.value, 100.0);

        let pointer = rows.iter().find(|r| r.hash == pointer_hash).unwrap();
        assert!(!pointer.editable);
        assert_eq!(pointer.value, 0xabcd as f64);
    }

    #[test]
    fn browse_labels_hashes_absent_from_the_dictionary_as_unknown() {
        let buf = buffer_with_slots(&[(0xdeadbeef, 42)]);
        let hash_table_end = buf.len();
        let rows = browse(&buf, hash_table_end).unwrap();
        assert_eq!(rows[0].name, None);
        assert_eq!(rows[0].type_name, "Unknown");
        assert!(!rows[0].editable);
    }

    #[test]
    fn write_scalar_round_trips_a_known_int_field() {
        let hash = hash32("PlayerStatus.MaxLife");
        let mut buf = buffer_with_slots(&[(hash, 100)]);
        let hash_table_end = buf.len();

        write_scalar(&mut buf, hash_table_end, hash, 400.0).unwrap();

        let rows = browse(&buf, hash_table_end).unwrap();
        assert_eq!(rows[0].value, 400.0);
    }

    #[test]
    fn write_scalar_rejects_pointer_fields() {
        let hash = hash32("Pouch.Weapon.ValidNum");
        let mut buf = buffer_with_slots(&[(hash, 0)]);
        let hash_table_end = buf.len();

        assert!(write_scalar(&mut buf, hash_table_end, hash, 1.0).is_err());
    }

    #[test]
    fn write_scalar_rejects_unknown_hashes() {
        let mut buf = buffer_with_slots(&[(0xdeadbeef, 0)]);
        let hash_table_end = buf.len();
        assert!(write_scalar(&mut buf, hash_table_end, 0xdeadbeef, 1.0).is_err());
    }
}
