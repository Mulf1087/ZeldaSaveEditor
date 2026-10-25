//! Key-item ability toggles (Ultrahand, Fuse, Ascend, Recall, Autobuild, Amiibo), matching the
//! six checkboxes marcrobledo shows on the TOTK key-items tab. Each ability is gated by two Bool
//! fields the game keeps in lockstep: `IsGet.Obj_<id>` and `IsGetAnyway.Obj_<id>`. An ability
//! reads as unlocked only when both are set, and toggling writes both together, exactly as the
//! source does.

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::hashtable::resolve_present_hashes;
use crate::totk::murmur3::hash32;

/// The six abilities in the order the source lists them, each with its in-game object id and the
/// friendly label shown to the player.
pub const ABILITIES: [(&str, &str); 6] = [
    ("UltraHand", "Ultrahand"),
    ("OneTouchBond", "Fuse"),
    ("Tooreroof", "Ascend"),
    ("ReverseRecorder", "Recall"),
    ("AutoBuilder", "Autobuild"),
    ("AmiiboItem", "Amiibo"),
];

pub struct AbilityState {
    pub id: String,
    pub label: String,
    pub unlocked: bool,
}

/// The `(IsGet, IsGetAnyway)` field hashes for one ability's object id.
fn flag_hashes(id: &str) -> (u32, u32) {
    (hash32(&format!("IsGet.Obj_{id}")), hash32(&format!("IsGetAnyway.Obj_{id}")))
}

pub fn read_abilities(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<AbilityState>, SaveError> {
    let hashes: Vec<u32> = ABILITIES
        .iter()
        .flat_map(|(id, _)| {
            let (a, b) = flag_hashes(id);
            [a, b]
        })
        .collect();
    let offsets = resolve_present_hashes(buf, hash_table_end, &hashes)?;

    let mut out = Vec::with_capacity(ABILITIES.len());
    for (id, label) in ABILITIES {
        let (a, b) = flag_hashes(id);
        let get = offsets.get(&a).map(|&o| buf.read_u32(o)).transpose()?.unwrap_or(0);
        let anyway = offsets.get(&b).map(|&o| buf.read_u32(o)).transpose()?.unwrap_or(0);
        out.push(AbilityState { id: id.to_string(), label: label.to_string(), unlocked: get != 0 && anyway != 0 });
    }
    Ok(out)
}

/// Sets both lockstep flags for `id` to `on`. Requires both fields to already be present in the
/// save's hash table (they coexist in any save past the intro); a missing field is an error
/// rather than a silent partial write that would leave the ability half-toggled.
pub fn set_ability(buf: &mut SaveBuffer, hash_table_end: usize, id: &str, on: bool) -> Result<(), SaveError> {
    let (a, b) = flag_hashes(id);
    let offsets = resolve_present_hashes(buf, hash_table_end, &[a, b])?;
    let value = if on { 1u32 } else { 0 };
    for h in [a, b] {
        let offset = *offsets.get(&h).ok_or(SaveError::MissingField("ability flag"))?;
        buf.write_u32(offset, value)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::totk::hashtable::HASH_TABLE_START;

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
    fn unlocked_only_when_both_flags_set() {
        let (get, anyway) = flag_hashes("UltraHand");
        // Only IsGet set: still reads as locked.
        let buf = buffer_with_slots(&[(get, 1), (anyway, 0)]);
        let end = buf.len();
        let ultrahand = read_abilities(&buf, end).unwrap().into_iter().find(|a| a.id == "UltraHand").unwrap();
        assert!(!ultrahand.unlocked);
    }

    #[test]
    fn set_ability_writes_both_flags() {
        let (get, anyway) = flag_hashes("Tooreroof");
        let mut buf = buffer_with_slots(&[(get, 0), (anyway, 0)]);
        let end = buf.len();
        set_ability(&mut buf, end, "Tooreroof", true).unwrap();
        let ascend = read_abilities(&buf, end).unwrap().into_iter().find(|a| a.id == "Tooreroof").unwrap();
        assert!(ascend.unlocked);
        set_ability(&mut buf, end, "Tooreroof", false).unwrap();
        let ascend = read_abilities(&buf, end).unwrap().into_iter().find(|a| a.id == "Tooreroof").unwrap();
        assert!(!ascend.unlocked);
    }

    #[test]
    fn set_ability_errors_when_flag_field_absent() {
        let buf_slots: &[(u32, u32)] = &[];
        let mut buf = buffer_with_slots(buf_slots);
        let end = buf.len();
        assert!(set_ability(&mut buf, end, "AmiiboItem", true).is_err());
    }
}
