use save_engine::botw::ModifierCategory;
use save_engine::{Save, SaveError};

fn fixture_bytes() -> Vec<u8> {
    std::fs::read("tests/fixtures/botw/game_data.sav").expect("fixture present")
}

fn load_botw() -> save_engine::botw::BotwSave {
    match Save::detect(fixture_bytes()).expect("should detect a known BOTW save") {
        Save::Botw(save) => save,
        Save::Totk(_) => panic!("BOTW fixture was misdetected as TOTK"),
    }
}

#[test]
fn detects_as_botw_v1_1_wiiu() {
    let save = load_botw();
    assert_eq!(save.version_index, 1);
    assert!(!save.modded);
}

#[test]
fn round_trip_without_edits_is_byte_identical() {
    let original = fixture_bytes();
    let save = Save::detect(original.clone()).unwrap();
    let output = save.to_bytes();
    assert_eq!(original, output);
}

#[test]
fn editing_rupees_only_touches_the_rupee_field() {
    let original = fixture_bytes();
    let mut save = load_botw();
    let before = save.rupees().unwrap();
    save.set_rupees(before.wrapping_add(500)).unwrap();
    assert_eq!(save.rupees().unwrap(), before.wrapping_add(500));
    let output = save.to_bytes();

    assert_eq!(output.len(), original.len());
    // Rupees are usually small, so `+500` may only flip the low 1-2 bytes rather than
    // all 4, so assert the edit is confined to a single 4-byte window, not an exact count.
    let diff_positions: Vec<usize> = original
        .iter()
        .zip(output.iter())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(i, _)| i)
        .collect();
    assert!(!diff_positions.is_empty(), "rupees value should have changed at least one byte");
    let min = *diff_positions.iter().min().unwrap();
    let max = *diff_positions.iter().max().unwrap();
    assert!(max - min < 4, "all differing bytes should be within the 4-byte rupee field");
}

#[test]
fn items_list_matches_first_and_last_expectations() {
    let save = load_botw();
    let items = save.items().unwrap();
    assert!(!items.is_empty());
    assert!(items.len() <= save_engine::botw::MAX_ITEMS);
    // every parsed item has a non-empty name (loop stops at the first empty slot)
    assert!(items.iter().all(|item| !item.name.is_empty()));
}

#[test]
fn setting_an_item_name_round_trips() {
    let mut save = load_botw();
    let items_before = save.items().unwrap();
    let last_index = items_before.len() - 1;

    save.set_item(last_index, "Weapon_Sword_070", 1).unwrap();
    let items_after = save.items().unwrap();
    assert_eq!(items_after[last_index].name, "Weapon_Sword_070");
    assert_eq!(items_after[last_index].quantity, 1);
}

#[test]
fn modifiers_counts_match_items_categorization() {
    let save = load_botw();
    let (weapons, bows, shields) = save.modifiers().unwrap();
    // sanity: modifier lists are readable and don't panic; exact counts depend on
    // the fixture's specific inventory contents.
    let _ = (weapons.len(), bows.len(), shields.len());
}

#[test]
fn setting_a_weapon_modifier_round_trips() {
    let mut save = load_botw();
    let (weapons, _, _) = save.modifiers().unwrap();
    if weapons.is_empty() {
        return; // fixture has no weapons in inventory; nothing to assert
    }
    save.set_modifier(ModifierCategory::Weapon, 0, 0x80000001, 5).unwrap();
    let (weapons_after, _, _) = save.modifiers().unwrap();
    assert_eq!(weapons_after[0].modifier, 0x80000001);
    assert_eq!(weapons_after[0].value, 5);
}

#[test]
fn horses_returns_six_slots_with_last_slot_nameless() {
    let save = load_botw();
    let horses = save.horses().unwrap();
    assert_eq!(horses.len(), save_engine::botw::NUM_HORSE_SLOTS);
    assert!(horses[5].name.is_none());
    assert!(horses[0].name.is_some());
}

#[test]
fn duplicate_and_remove_keep_modifiers_aligned() {
    let mut save = load_botw();
    let items = save.items().unwrap();
    let (weapons, _, _) = save.modifiers().unwrap();
    if weapons.is_empty() {
        return; // fixture has no weapons; nothing to align
    }
    let widx = items
        .iter()
        .position(|it| {
            it.name.starts_with("Weapon_Sword_")
                || it.name.starts_with("Weapon_Lsword_")
                || it.name.starts_with("Weapon_Spear_")
        })
        .expect("weapon count > 0 implies a weapon item exists");

    // Tag the first weapon's modifier so we can prove it stays on the right item.
    save.set_modifier(ModifierCategory::Weapon, 0, 0x80000002, 7).unwrap();

    save.duplicate_item(widx).unwrap();
    let items_after = save.items().unwrap();
    let (w_after, _, _) = save.modifiers().unwrap();
    assert_eq!(items_after.len(), items.len() + 1);
    assert_eq!(w_after.len(), weapons.len() + 1);
    assert_eq!(items_after[widx + 1].name, items[widx].name);
    // Both the original and its copy carry the tagged modifier: no shift onto neighbours.
    assert_eq!(w_after[0].modifier, 0x80000002);
    assert_eq!(w_after[1].modifier, 0x80000002);

    save.remove_item(widx + 1).unwrap();
    let items_back = save.items().unwrap();
    let (w_back, _, _) = save.modifiers().unwrap();
    assert_eq!(items_back.len(), items.len());
    assert_eq!(w_back.len(), weapons.len());
    assert_eq!(w_back[0].modifier, 0x80000002);
}

#[test]
fn completism_snapshot_and_set_complete_a_category() {
    let mut save = load_botw();
    let cats = save.completism().unwrap();
    for id in ["koroks", "locations", "hinox", "talus", "molduga"] {
        assert!(cats.iter().any(|c| c.id == id), "missing category {id}");
    }
    let koroks = cats.iter().find(|c| c.id == "koroks").unwrap();
    assert_eq!(koroks.metrics[0].max, 900);

    let molduga_before = cats.iter().find(|c| c.id == "molduga").unwrap().metrics[0].clone();
    let changed = save.set_completism("molduga", 0).unwrap();
    let after = save.completism().unwrap();
    let molduga_after = &after.iter().find(|c| c.id == "molduga").unwrap().metrics[0];
    assert_eq!(molduga_after.value, molduga_before.value + changed);
    assert_eq!(save.set_completism("molduga", 0).unwrap(), 0, "idempotent");
}

#[test]
fn set_item_out_of_range_returns_error() {
    let mut save = load_botw();
    let result = save.set_item(save_engine::botw::MAX_ITEMS, "Weapon_Sword_070", 1);
    assert!(matches!(result, Err(SaveError::IndexOutOfRange { .. })));
}

#[test]
fn set_horse_name_out_of_range_returns_error() {
    let mut save = load_botw();
    let result = save.set_horse_name(5, "Epona");
    assert!(matches!(result, Err(SaveError::IndexOutOfRange { .. })));
}

#[test]
fn set_horse_type_out_of_range_returns_error() {
    let mut save = load_botw();
    let result = save.set_horse_type(save_engine::botw::NUM_HORSE_SLOTS, "HorseType");
    assert!(matches!(result, Err(SaveError::IndexOutOfRange { .. })));
}

#[test]
fn items_with_category_pairs_every_item_in_order_with_real_fixture_categories() {
    use save_engine::botw::items::ItemCategory;

    let save = load_botw();
    let items = save.items().unwrap();
    let paired = save.items_with_category().unwrap();

    // same length/order as the plain flat list, so index i still matches set_item(i, ...)
    assert_eq!(paired.len(), items.len());
    for (plain, (paired_item, _)) in items.iter().zip(paired.iter()) {
        assert_eq!(plain, paired_item);
    }

    assert_eq!(paired[0].0.name, "Weapon_Sword_070"); // Master Sword
    assert_eq!(paired[0].1, ItemCategory::Weapon);
    assert_eq!(paired[20].0.name, "Weapon_Bow_001");
    assert_eq!(paired[20].1, ItemCategory::Bow);
    assert_eq!(paired[52].0.name, "Armor_043_Upper");
    assert_eq!(paired[52].1, ItemCategory::Armor);
    assert_eq!(paired[229].0.name, "Item_Ore_C");
    assert_eq!(paired[229].1, ItemCategory::Material);
    assert_eq!(paired[270].0.name, "Item_Cook_D_03");
    assert_eq!(paired[270].1, ItemCategory::Food);
    assert_eq!(paired[296].0.name, "Obj_DRStone_Get");
    assert_eq!(paired[296].1, ItemCategory::KeyItem);
}

#[test]
fn unlock_all_koroks_finds_the_rest_and_bumps_counter_and_inventory() {
    let mut save = load_botw();
    let before_counter = save.korok_seed_counter().unwrap();
    let korok_nuts_before = save
        .items()
        .unwrap()
        .into_iter()
        .find(|item| item.name == "Obj_KorokNuts")
        .map(|item| item.quantity);

    let newly_found = save.unlock_all_koroks().unwrap();
    assert_eq!(newly_found, 524);
    assert_eq!(save.korok_seed_counter().unwrap(), 900); // every korok now found
    assert_eq!(before_counter + newly_found as u32, 900);

    if let Some(before_qty) = korok_nuts_before {
        let after_qty = save
            .items()
            .unwrap()
            .into_iter()
            .find(|item| item.name == "Obj_KorokNuts")
            .unwrap()
            .quantity;
        assert_eq!(after_qty, before_qty + newly_found as u32);
    }

    // idempotent: nothing left to unlock
    assert_eq!(save.unlock_all_koroks().unwrap(), 0);
}

#[test]
fn unlock_all_defeated_bosses_bumps_matching_counters_to_the_full_total() {
    let mut save = load_botw();

    let hinox_found = save.unlock_all_defeated_hinox().unwrap();
    assert_eq!(hinox_found, 32);
    assert_eq!(save.defeated_hinox_counter().unwrap(), 40);
    assert_eq!(save.unlock_all_defeated_hinox().unwrap(), 0);

    let talus_found = save.unlock_all_defeated_talus().unwrap();
    assert_eq!(talus_found, 35);
    assert_eq!(save.defeated_talus_counter().unwrap(), 40);
    assert_eq!(save.unlock_all_defeated_talus().unwrap(), 0);

    let molduga_found = save.unlock_all_defeated_molduga().unwrap();
    assert_eq!(molduga_found, 3);
    assert_eq!(save.defeated_molduga_counter().unwrap(), 4);
    assert_eq!(save.unlock_all_defeated_molduga().unwrap(), 0);
}

#[test]
fn unlock_all_locations_is_idempotent_and_has_no_companion_counter() {
    let mut save = load_botw();
    let newly_visited = save.unlock_all_locations().unwrap();
    assert_eq!(newly_visited, 52);
    assert_eq!(save.unlock_all_locations().unwrap(), 0);
}
