//! MurmurHash3 x86_32 (seed 0), the canonical Austin Appleby algorithm, ported from the
//! shape and magic constants in the real `zelda-totk` source's vendored
//! `lib/murmurhash3js.min.js` (MIT licensed). Unlike the core slice's hashes (already literal
//! `u32` constants in `zelda-totk.js`), Pouch item and horse field hashes are computed at
//! runtime from name strings in the source (`zelda-totk.variables.js`'s `Variable` constructor
//! calls `murmurHash3.x86.hash32(hashText)`), so this crate needs the actual algorithm rather
//! than a list of pre-computed constants, verified via 3 independent cross-checks
//! (already-shipped Rust constants, a community-published literal hash CSV, and live decoding
//! of the real fixture).

const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;

pub fn hash32(text: &str) -> u32 {
    let bytes = text.as_bytes();
    let mut h: u32 = 0;

    let chunks = bytes.chunks_exact(4);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let k = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        h ^= mix_k(k);
        h = h.rotate_left(13);
        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let mut k: u32 = 0;
    for (i, &b) in remainder.iter().enumerate().rev() {
        k ^= (b as u32) << (8 * i);
    }
    if !remainder.is_empty() {
        h ^= mix_k(k);
    }

    h ^= bytes.len() as u32;
    finalize(h)
}

fn mix_k(k: u32) -> u32 {
    let k = k.wrapping_mul(C1);
    let k = k.rotate_left(15);
    k.wrapping_mul(C2)
}

fn finalize(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    // Cross-check 1: already-shipped literal hash constants in `totk::mod::HASHES`
    // (crates/save-engine/src/totk/mod.rs). These were taken directly from
    // `zelda-totk.js`'s literal `Hashes` array and are independently known-correct.
    #[test]
    fn matches_already_shipped_pouch_valid_num_hashes() {
        assert_eq!(hash32("Pouch.Weapon.ValidNum"), 0xd7a3f6ba);
        assert_eq!(hash32("Pouch.Bow.ValidNum"), 0xc61785c2);
        assert_eq!(hash32("Pouch.Shield.ValidNum"), 0x05271e7d);
    }

    // Cross-check 2: community-published literal hash CSV
    // (`zelda-totk/zelda-totk.hashes.csv`, "cracked and compiled by MacSpazzy and MrCheeze",
    // vendored alongside the JS source in the same repo the progress.sav fixture comes from).
    #[test]
    fn matches_community_hash_csv() {
        assert_eq!(hash32("OwnedHorseList.ActorName"), 0x7bde80e9);
        assert_eq!(hash32("Pouch.SpecialParts.Content.Name"), 0xa86f2f10);
        assert_eq!(hash32("Pouch.Armor.Content.ColorVariation"), 0x183e2a32);
        assert_eq!(hash32("Pouch.Weapon.Content.Name"), 0x65efd0be);
    }

    // Cross-check 3: this hash, decoded live against the real fixture, resolves to an
    // address whose first 4 bytes (every armor slot's dye color in progress.sav) equal
    // this exact value, i.e. "no dye applied", the default.
    #[test]
    fn matches_live_fixture_decoded_dye_default() {
        assert_eq!(hash32("None"), 0xb6eede09);
    }

    // Cross-check 4: completionism status-value names, hashed by the source's bundled
    // `murmurHash3.x86.hash32` (used to generate `completism_data.rs`). Guards the generated
    // data against any drift between this algorithm and the one that produced those constants.
    #[test]
    fn matches_completism_status_value_hashes() {
        assert_eq!(hash32("Clear"), 0x62965740);
        assert_eq!(hash32("Open"), 0x1818ec02);
        assert_eq!(hash32("Unopened"), 0x8d96a2c5);
        assert_eq!(hash32("Buy"), 0xbedf2a35);
    }

    #[test]
    fn empty_string_does_not_panic() {
        // exercises the zero-length remainder path
        let _ = hash32("");
    }
}
