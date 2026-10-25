use save_engine::totk::pouch;
use save_engine::Save;

fn fixture_bytes() -> Vec<u8> {
    std::fs::read("tests/fixtures/totk/progress.sav").expect("fixture present")
}

fn load_totk() -> save_engine::totk::TotkSave {
    match Save::detect(fixture_bytes()).expect("should detect a known TOTK save") {
        Save::Totk(save) => save,
        Save::Botw(_) => panic!("TOTK fixture was misdetected as BOTW"),
    }
}

#[test]
fn detects_as_totk_v1_1_x_v1_2_x() {
    let save = load_totk();
    assert_eq!(save.version_label, "v1.1.x/v1.2.x");
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
fn editing_current_rupees_only_touches_the_rupee_field() {
    let original = fixture_bytes();
    let mut save = load_totk();
    let before = save.current_rupees().unwrap();
    save.set_current_rupees(before.wrapping_add(500)).unwrap();
    assert_eq!(save.current_rupees().unwrap(), before.wrapping_add(500));
    let output = save.to_bytes();

    assert_eq!(output.len(), original.len());
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
fn setting_save_pos_round_trips_through_pointer_indirection() {
    let mut save = load_totk();
    save.set_save_pos(100.5, 20.25, -300.75).unwrap();
    let (x, y, z) = save.save_pos().unwrap();
    assert_eq!((x, y, z), (100.5, 20.25, -300.75));
}

#[test]
fn setting_sequence_current_banc_round_trips_through_pointer_indirection() {
    let mut save = load_totk();
    save.set_sequence_current_banc("TestBanc001").unwrap();
    assert_eq!(save.sequence_current_banc().unwrap(), "TestBanc001");
}

#[test]
fn pouch_valid_num_accessors_round_trip() {
    let mut save = load_totk();
    save.set_pouch_weapon_valid_num(5).unwrap();
    save.set_pouch_bow_valid_num(3).unwrap();
    save.set_pouch_shield_valid_num(2).unwrap();
    assert_eq!(save.pouch_weapon_valid_num().unwrap(), 5);
    assert_eq!(save.pouch_bow_valid_num().unwrap(), 3);
    assert_eq!(save.pouch_shield_valid_num().unwrap(), 2);
}

#[test]
fn pouch_weapons_reads_real_fixture_contents() {
    let save = load_totk();
    let weapons = save.pouch_weapons().unwrap();
    assert_eq!(weapons.len(), 2);
    assert_eq!(weapons[0].id, "Weapon_Sword_070"); // Master Sword
    assert_eq!(weapons[0].durability, 40);
    assert_eq!(weapons[0].fuse_id, "");
    assert_eq!(weapons[1].id, "Weapon_Sword_166"); // Gloom Sword
    assert_eq!(weapons[1].durability, 40);
    assert_eq!(weapons[1].fuse_id, "Item_Enemy_226");
}

#[test]
fn set_pouch_weapons_round_trips_and_updates_valid_num() {
    let mut save = load_totk();
    let mut weapons = save.pouch_weapons().unwrap();
    weapons.push(pouch::WeaponEntry {
        id: "Weapon_Sword_001".into(),
        durability: 20,
        modifier: 0,
        modifier_value: 0,
        fuse_id: String::new(),
        fuse_durability: 0,
        extra_durability: 0,
        record_extra_durability: -1,
    });
    save.set_pouch_weapons(&weapons).unwrap();
    assert_eq!(save.pouch_weapon_valid_num().unwrap(), 3);
    let reloaded = save.pouch_weapons().unwrap();
    assert_eq!(reloaded.len(), 3);
    assert_eq!(reloaded[2].id, "Weapon_Sword_001");
}

#[test]
fn pouch_bows_reads_real_fixture_contents() {
    let save = load_totk();
    let bows = save.pouch_bows().unwrap();
    assert_eq!(bows.len(), 2);
    assert_eq!(bows[0].id, "Weapon_Bow_036");
    assert_eq!(bows[0].durability, 55);
    assert_eq!(bows[0].modifier, 0xdad10617);
    assert_eq!(bows[0].modifier_value, 10);
    assert_eq!(bows[1].id, "Weapon_Bow_013");
    assert_eq!(bows[1].durability, 17);
    assert_eq!(bows[1].modifier, 0xb6eede09); // hash32("None"): no modifier
    assert_eq!(bows[1].modifier_value, 0);
}

#[test]
fn set_pouch_bows_round_trips_and_updates_valid_num() {
    let mut save = load_totk();
    let mut bows = save.pouch_bows().unwrap();
    bows.push(pouch::BowEntry {
        id: "Weapon_Bow_001".into(),
        durability: 20,
        modifier: 0,
        modifier_value: 0,
    });
    save.set_pouch_bows(&bows).unwrap();
    assert_eq!(save.pouch_bow_valid_num().unwrap(), 3);
    let reloaded = save.pouch_bows().unwrap();
    assert_eq!(reloaded.len(), 3);
    assert_eq!(reloaded[2].id, "Weapon_Bow_001");
}

#[test]
fn pouch_shields_reads_real_fixture_contents() {
    let save = load_totk();
    let shields = save.pouch_shields().unwrap();
    assert_eq!(shields.len(), 2);
    assert_eq!(shields[0].id, "Weapon_Shield_057");
    assert_eq!(shields[0].durability, 90);
    assert_eq!(shields[0].modifier, 0xb6eede09); // hash32("None"): no modifier
    assert_eq!(shields[0].fuse_id, "");
    assert_eq!(shields[0].fuse_durability, -1);
    assert_eq!(shields[1].id, "Weapon_Shield_022");
    assert_eq!(shields[1].durability, 29);
    assert_eq!(shields[1].modifier, 0xb3c94e5);
    assert_eq!(shields[1].modifier_value, 6);
}

#[test]
fn set_pouch_shields_round_trips_and_updates_valid_num() {
    let mut save = load_totk();
    let mut shields = save.pouch_shields().unwrap();
    shields.push(pouch::ShieldEntry {
        id: "Weapon_Shield_001".into(),
        durability: 20,
        modifier: 0,
        modifier_value: 0,
        fuse_id: String::new(),
        fuse_durability: 0,
        extra_durability: 0,
    });
    save.set_pouch_shields(&shields).unwrap();
    assert_eq!(save.pouch_shield_valid_num().unwrap(), 3);
    let reloaded = save.pouch_shields().unwrap();
    assert_eq!(reloaded.len(), 3);
    assert_eq!(reloaded[2].id, "Weapon_Shield_001");
}

#[test]
fn pouch_armor_reads_real_fixture_contents() {
    let save = load_totk();
    let armor = save.armor().unwrap();
    assert_eq!(armor.len(), 87);
    assert_eq!(armor[0].id, "Armor_1043_Upper"); // Archaic Tunic
    assert_eq!(armor[0].dye_color, save_engine::totk::murmur3::hash32("None"));
    assert_eq!(armor[3].id, "Armor_003_Head");
}

#[test]
fn arrows_round_trip_scanning_until_empty_id() {
    let mut save = load_totk();
    let mut arrows = save.arrows().unwrap();
    arrows.push(pouch::ArrowEntry {
        id: "NormalArrow".into(),
        quantity: 999,
    });
    save.set_arrows(&arrows).unwrap();
    let reloaded = save.arrows().unwrap();
    assert_eq!(reloaded.len(), arrows.len());
    assert_eq!(reloaded.last().unwrap().id, "NormalArrow");
    assert_eq!(reloaded.last().unwrap().quantity, 999);
}

#[test]
fn materials_round_trip_scanning_until_empty_id() {
    let mut save = load_totk();
    let mut materials = save.materials().unwrap();
    materials.push(pouch::MaterialEntry {
        id: "Item_Fruit_A".into(),
        quantity: 5,
        get_order: 42,
        use_order: 7,
    });
    save.set_materials(&materials).unwrap();
    let reloaded = save.materials().unwrap();
    assert_eq!(reloaded.len(), materials.len());
    let last = reloaded.last().unwrap();
    assert_eq!(last.id, "Item_Fruit_A");
    assert_eq!(last.quantity, 5);
    assert_eq!(last.get_order, 42);
    assert_eq!(last.use_order, 7);
}

#[test]
fn key_items_round_trip_and_allow_negative_one_quantity() {
    let mut save = load_totk();
    let mut key_items = save.key_items().unwrap();
    key_items.push(pouch::KeyItemEntry {
        id: "PouchExtension_01".into(),
        quantity: -1,
    });
    save.set_key_items(&key_items).unwrap();
    let reloaded = save.key_items().unwrap();
    assert_eq!(reloaded.len(), key_items.len());
    let last = reloaded.last().unwrap();
    assert_eq!(last.id, "PouchExtension_01");
    assert_eq!(last.quantity, -1);
}

#[test]
fn devices_reads_real_fixture_zonai_inventory() {
    let save = load_totk();
    let devices = save.devices().unwrap();
    assert_eq!(devices[0].id, "SpObj_WindGenerator_Capsule_A_01"); // Fan
    assert_eq!(devices[0].quantity, 99);
    assert_eq!(devices[4].id, "SpObj_Rocket_Capsule_A_01"); // Rocket
    assert_eq!(devices[4].quantity, 72);
}

#[test]
fn devices_round_trip_scanning_until_empty_id() {
    let mut save = load_totk();
    let mut devices = save.devices().unwrap();
    devices.push(pouch::DeviceEntry {
        id: "SpObj_Fan_Capsule_A_01".into(),
        quantity: 3,
        use_order: 1,
    });
    save.set_devices(&devices).unwrap();
    let reloaded = save.devices().unwrap();
    assert_eq!(reloaded.len(), devices.len());
    let last = reloaded.last().unwrap();
    assert_eq!(last.id, "SpObj_Fan_Capsule_A_01");
    assert_eq!(last.quantity, 3);
    assert_eq!(last.use_order, 1);
}

#[test]
fn food_reads_real_fixture_recipe_partition() {
    let save = load_totk();
    let food = save.food().unwrap();
    assert_eq!(food[0].id, "Item_Cook_C_17"); // Elixir
    assert_eq!(food[0].quantity, 1);
    assert_eq!(
        food[0].recipe,
        [
            "Animal_Insect_M".to_string(),
            "Item_Enemy_40".to_string(),
            "Item_Mushroom_L".to_string(),
            "".to_string(),
            "".to_string(),
        ]
    );
}

#[test]
fn food_round_trips_including_recipe_partition() {
    let mut save = load_totk();
    let mut food = save.food().unwrap();
    food.push(pouch::FoodEntry {
        id: "Item_Fruit_A".into(),
        quantity: 2,
        hearts_heal: 4,
        effect: save_engine::totk::murmur3::hash32("None"),
        effect_multiplier: 0,
        effect_time: 0,
        price: 10,
        recipe: [
            "Item_Fruit_A".to_string(),
            "Item_Fruit_A".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ],
    });
    save.set_food(&food).unwrap();
    let reloaded = save.food().unwrap();
    assert_eq!(reloaded.len(), food.len());
    let last = reloaded.last().unwrap();
    assert_eq!(last.id, "Item_Fruit_A");
    assert_eq!(last.hearts_heal, 4);
    assert_eq!(last.price, 10);
    assert_eq!(
        last.recipe,
        [
            "Item_Fruit_A".to_string(),
            "Item_Fruit_A".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]
    );
    // Untouched fixture entries' recipes must still read back correctly after the write pass.
    assert_eq!(
        reloaded[0].recipe,
        [
            "Animal_Insect_M".to_string(),
            "Item_Enemy_40".to_string(),
            "Item_Mushroom_L".to_string(),
            "".to_string(),
            "".to_string(),
        ]
    );
}

#[test]
fn horses_reads_real_fixture_records() {
    let save = load_totk();
    let horses = save.horses().unwrap();
    assert_eq!(horses.len(), 6);
    assert_eq!(horses[0].id, "GameRomHorse04");
    assert_eq!(horses[0].name, "Max");
    assert_eq!(horses[0].bond, 1.0);
    assert_eq!(horses[0].room_id, -1);
    assert_eq!(horses[0].amiibo_uid_hash, 0);

    assert_eq!(horses[1].name, "Brownie");
    assert_eq!(horses[3].id, "GameRomHorse01L"); // giant white stallion
    assert_eq!(horses[3].name, "Zelda");
    assert!((horses[3].bond - 0.67).abs() < 0.01);
    assert_eq!(horses[5].id, "GameRomHorseGold");
    assert_eq!(horses[5].name, "Dorado");
}

#[test]
fn set_horses_round_trips_name_and_bond() {
    let mut save = load_totk();
    let mut horses = save.horses().unwrap();
    horses[0].name = "Renamed".to_string();
    horses[0].bond = 0.5;
    save.set_horses(&horses).unwrap();
    let reloaded = save.horses().unwrap();
    assert_eq!(reloaded[0].name, "Renamed");
    assert_eq!(reloaded[0].bond, 0.5);
}

#[test]
fn set_horses_round_trips_every_field_catching_stride_or_tuple_order_bugs() {
    let mut save = load_totk();
    let mut horses = save.horses().unwrap();
    horses[0] = save_engine::totk::horse::HorseEntry {
        id: horses[0].id.clone(),
        name: "FullFieldTest".to_string(),
        mane: 0x11111111,
        saddle: 0x22222222,
        rein: 0x33333333,
        bond: 0.42,
        bond_checked: true,
        stats_strength: 101,
        stats_speed: 102,
        stats_stamina: 103,
        stats_pull: 104,
        horse_type: 7,
        color_type: 8,
        foot_type: 9,
        amiibo_uid_hash: 0x0123456789abcdef,
        room_id: 55,
        icon_pattern: 0x44444444,
        icon_eye_color: 0x55555555,
        icon_primary_color: (1, 2, 3),
        icon_secondary_color: (4, 5, 6),
        icon_nose_color: (7, 8, 9),
        icon_hair_primary_color: (10, 11, 12),
        icon_hair_secondary_color: (13, 14, 15),
    };
    save.set_horses(&horses).unwrap();
    let reloaded = save.horses().unwrap();
    let h = &reloaded[0];
    assert_eq!(h.name, "FullFieldTest");
    assert_eq!(h.mane, 0x11111111);
    assert_eq!(h.saddle, 0x22222222);
    assert_eq!(h.rein, 0x33333333);
    assert_eq!(h.bond, 0.42);
    assert_eq!(h.bond_checked, true);
    assert_eq!(h.stats_strength, 101);
    assert_eq!(h.stats_speed, 102);
    assert_eq!(h.stats_stamina, 103);
    assert_eq!(h.stats_pull, 104);
    assert_eq!(h.horse_type, 7);
    assert_eq!(h.color_type, 8);
    assert_eq!(h.foot_type, 9);
    assert_eq!(h.amiibo_uid_hash, 0x0123456789abcdef);
    assert_eq!(h.room_id, 55);
    assert_eq!(h.icon_pattern, 0x44444444);
    assert_eq!(h.icon_eye_color, 0x55555555);
    assert_eq!(h.icon_primary_color, (1, 2, 3));
    assert_eq!(h.icon_secondary_color, (4, 5, 6));
    assert_eq!(h.icon_nose_color, (7, 8, 9));
    assert_eq!(h.icon_hair_primary_color, (10, 11, 12));
    assert_eq!(h.icon_hair_secondary_color, (13, 14, 15));
}

#[test]
fn set_pouch_weapons_beyond_capacity_returns_err() {
    let mut save = load_totk();
    let base = save.pouch_weapons().unwrap();
    let template = pouch::WeaponEntry {
        id: "Weapon_Sword_070".into(),
        durability: 40,
        modifier: 0,
        modifier_value: 0,
        fuse_id: String::new(),
        fuse_durability: 0,
        extra_durability: 0,
        record_extra_durability: -1,
    };
    // Real fixture's weapon array capacity is 40; 41 entries must be rejected.
    let mut too_many: Vec<pouch::WeaponEntry> = Vec::new();
    for _ in 0..41 {
        too_many.push(pouch::WeaponEntry {
            id: template.id.clone(),
            durability: template.durability,
            modifier: template.modifier,
            modifier_value: template.modifier_value,
            fuse_id: template.fuse_id.clone(),
            fuse_durability: template.fuse_durability,
            extra_durability: template.extra_durability,
            record_extra_durability: template.record_extra_durability,
        });
    }
    let result = save.set_pouch_weapons(&too_many);
    assert!(matches!(result, Err(save_engine::SaveError::IndexOutOfRange { .. })));
    // Sanity: base fixture read still works (save untouched by the failed write attempt's error path).
    assert_eq!(base.len(), 2);
}

#[test]
fn malformed_input_returns_err_instead_of_panicking() {
    // Right magic and in-range size, but all-zero content: no sentinel hash exists in
    // it, so the hash-table-end scan should fail gracefully rather than panic or scan
    // forever.
    let mut bytes = vec![0u8; 2307552];
    bytes[0..4].copy_from_slice(&0x01020304u32.to_le_bytes());
    let result = Save::detect(bytes);
    assert!(result.is_err());
}

#[test]
fn completism_counts_are_read_only_and_match_real_fixture_contents() {
    let save = load_totk();
    assert_eq!(save.shrines_found().unwrap(), 152); // every shrine found in this fixture
    assert_eq!(save.shrines_cleared().unwrap(), 144);
    assert_eq!(save.koroks_hidden().unwrap(), 341);
    assert_eq!(save.koroks_carried().unwrap(), 33);
    assert_eq!(save.locations_visited().unwrap(), 285); // of 362 tracked locations
    assert_eq!(save.defeated_hinox().unwrap(), 18);
    assert_eq!(save.defeated_talus().unwrap(), 16);
    assert_eq!(save.defeated_molduga().unwrap(), 1);
}

#[test]
fn completism_snapshot_exposes_every_category_with_source_totals() {
    let save = load_totk();
    let cats = save.completism().unwrap();
    let by_id = |id: &str| cats.iter().find(|c| c.id == id).expect("category present");

    // Two-metric cards and their curated totals, straight from the source's arrays.
    let towers = by_id("towers");
    assert_eq!(towers.metrics.len(), 2);
    assert_eq!(towers.metrics[0].max, 15);
    assert_eq!(by_id("shrines").metrics[1].max, 152);
    assert_eq!(by_id("lightroots").metrics[0].max, 120);
    // Categories that only exist after this port.
    assert_eq!(by_id("caves").metrics[0].max, 147);
    assert_eq!(by_id("wells").metrics[0].max, 58);
    assert_eq!(by_id("chasms").metrics[0].max, 36);
    assert_eq!(by_id("compendium").metrics[0].max, 509);
    assert_eq!(by_id("gleeok").metrics[0].max, 14);
}

#[test]
fn set_completism_flips_a_category_to_complete() {
    let mut save = load_totk();
    let before = save.completism().unwrap();
    let hinox_before = before.iter().find(|c| c.id == "hinox").unwrap().metrics[0].clone();
    assert!(hinox_before.value < hinox_before.max, "fixture should start with hinoxes left");

    let changed = save.set_completism("hinox", 0).unwrap();
    assert!(changed > 0, "some present hinox flags should flip");

    let after = save.completism().unwrap();
    let hinox_after = &after.iter().find(|c| c.id == "hinox").unwrap().metrics[0];
    // Every flipped flag raises the count by exactly one; a second run is then a no-op.
    assert_eq!(hinox_after.value, hinox_before.value + changed);
    assert_eq!(save.set_completism("hinox", 0).unwrap(), 0, "idempotent");
}

#[test]
fn autobuilds_reads_real_fixture_contents() {
    let save = load_totk();
    let entries = save.autobuilds().unwrap();
    assert_eq!(entries.len(), 30);
    assert!(entries.iter().all(|e| e.combined_actor_info.len() == 6688));

    // every slot in this fixture is populated with a distinct in-game schematic index
    let mut indices: Vec<i32> = entries.iter().map(|e| e.index).collect();
    indices.sort_unstable();
    assert_eq!(indices, (0..30).collect::<Vec<i32>>());

    assert_eq!(entries[0].index, 29);
    assert!(entries[0].is_favorite);
    assert!((entries[0].camera_pos.1 - 32.223396).abs() < 0.001);
    assert!(!entries[1].is_favorite);

    let favorite_count = entries.iter().filter(|e| e.is_favorite).count();
    assert_eq!(favorite_count, 7);
}

#[test]
fn set_autobuilds_round_trips_index_camera_and_favorite() {
    let mut save = load_totk();
    let mut entries = save.autobuilds().unwrap();
    entries[0].index = -1;
    entries[0].camera_pos = (1.0, 2.0, 3.0);
    entries[0].is_favorite = !entries[0].is_favorite;
    let new_favorite = entries[0].is_favorite;

    save.set_autobuilds(&entries).unwrap();

    let reloaded = save.autobuilds().unwrap();
    assert_eq!(reloaded[0].index, -1);
    assert_eq!(reloaded[0].camera_pos, (1.0, 2.0, 3.0));
    assert_eq!(reloaded[0].is_favorite, new_favorite);
    // untouched slots stay untouched
    assert_eq!(reloaded[1].index, entries[1].index);
}

#[test]
fn set_autobuilds_rejects_wrong_combined_actor_info_length() {
    let mut save = load_totk();
    let mut entries = save.autobuilds().unwrap();
    entries[0].combined_actor_info = vec![0u8; 10]; // not 6688
    let result = save.set_autobuilds(&entries);
    assert!(matches!(result, Err(save_engine::SaveError::SizeMismatch { .. })));
}

#[test]
fn set_autobuilds_rejects_wrong_entry_count() {
    let mut save = load_totk();
    let mut entries = save.autobuilds().unwrap();
    entries.pop();
    let result = save.set_autobuilds(&entries);
    assert!(matches!(result, Err(save_engine::SaveError::SizeMismatch { .. })));
}

#[test]
fn map_pins_reads_real_fixture_contents() {
    use save_engine::totk::mapdata::ICON_NONE;

    let save = load_totk();
    let pins = save.map_pins().unwrap();
    assert_eq!(pins.len(), 300);

    let used: Vec<_> = pins.iter().filter(|p| !p.is_free()).collect();
    assert_eq!(used.len(), 289);
    assert!(!used.iter().any(|p| p.icon == ICON_NONE));

    // MapPin.ICON_LEAF / MapPin.MAP_MAIN in the source
    assert_eq!(pins[0].icon, 0x51b0bed0);
    assert_eq!(pins[0].layer, 0x24950135);
    assert!((pins[0].x - 3088.6).abs() < 0.1);
}

#[test]
fn set_map_pins_round_trips_and_can_clear_a_pin() {
    use save_engine::totk::mapdata::ICON_NONE;

    let mut save = load_totk();
    let mut pins = save.map_pins().unwrap();
    assert!(!pins[0].is_free());

    pins[0].icon = ICON_NONE;
    pins[0].x = 0.0;
    pins[0].y = 0.0;
    let untouched_x = pins[1].x;

    save.set_map_pins(&pins).unwrap();

    let reloaded = save.map_pins().unwrap();
    assert!(reloaded[0].is_free());
    assert_eq!(reloaded[1].x, untouched_x);
}

#[test]
fn set_map_pins_rejects_wrong_entry_count() {
    let mut save = load_totk();
    let mut pins = save.map_pins().unwrap();
    pins.pop();
    let result = save.set_map_pins(&pins);
    assert!(matches!(result, Err(save_engine::SaveError::SizeMismatch { .. })));
}

#[test]
fn guid_completism_counts_match_real_fixture_contents() {
    let save = load_totk();
    assert_eq!(save.defeated_bubbuls().unwrap(), 104);
    assert_eq!(save.sage_wills_found().unwrap(), 18);
    assert_eq!(save.old_maps_found().unwrap(), 31);
    assert_eq!(save.addison_completed().unwrap(), 37);
}

#[test]
fn map_markers_read_real_fixture_contents() {
    let save = load_totk();
    let markers = save.map_markers().unwrap();
    assert_eq!(markers.len(), 6);
    assert!((markers[0].x - 3650.0803).abs() < 0.01);
}

#[test]
fn set_map_markers_round_trips_and_rejects_wrong_entry_count() {
    let mut save = load_totk();
    let mut markers = save.map_markers().unwrap();
    markers[0].color = save_engine::totk::mapdata::ICON_NONE;
    markers[0].x = 0.0;
    save.set_map_markers(&markers).unwrap();
    let reloaded = save.map_markers().unwrap();
    assert!(reloaded[0].is_free());

    markers.pop();
    let result = save.set_map_markers(&markers);
    assert!(matches!(result, Err(save_engine::SaveError::SizeMismatch { .. })));
}

#[test]
fn unlock_all_bubbuls_adds_missing_guids_and_others_are_untouched() {
    let mut save = load_totk();
    let before_sage_wills = save.sage_wills_found().unwrap();
    let before_pouch = save.pouch_weapons().unwrap();
    let before_horses = save.horses().unwrap();
    let before_map_pins = save.map_pins().unwrap();

    let newly_unlocked = save.unlock_all_bubbuls().unwrap();

    assert!(newly_unlocked > 0, "fixture should have at least one undiscovered bubbul to unlock");
    assert_eq!(save.defeated_bubbuls().unwrap(), 147, "all 147 bubbuls should now read as defeated");
    assert_eq!(save.sage_wills_found().unwrap(), before_sage_wills, "unrelated GUID category must be untouched");
    assert_eq!(save.pouch_weapons().unwrap().len(), before_pouch.len(), "pouch weapons must survive the buffer growth unmoved");
    for (before, after) in before_pouch.iter().zip(save.pouch_weapons().unwrap().iter()) {
        assert_eq!(before.id, after.id);
        assert_eq!(before.durability, after.durability);
    }
    assert_eq!(save.horses().unwrap().len(), before_horses.len());
    assert_eq!(save.horses().unwrap()[0].name, before_horses[0].name);
    assert_eq!(save.map_pins().unwrap().len(), before_map_pins.len());
}

#[test]
fn unlock_all_bubbuls_is_idempotent() {
    let mut save = load_totk();
    save.unlock_all_bubbuls().unwrap();
    let second_run = save.unlock_all_bubbuls().unwrap();
    assert_eq!(second_run, 0, "nothing left to unlock on a second call");
}

#[test]
fn unlock_all_sage_wills_and_addison_work_and_stay_isolated() {
    let mut save = load_totk();
    let before_bubbuls = save.defeated_bubbuls().unwrap();

    let sage_wills_unlocked = save.unlock_all_sage_wills().unwrap();
    assert_eq!(save.sage_wills_found().unwrap(), 20);
    assert!(sage_wills_unlocked > 0);

    let addison_unlocked = save.unlock_all_addison().unwrap();
    assert_eq!(save.addison_completed().unwrap(), 81);
    assert!(addison_unlocked > 0);

    assert_eq!(save.defeated_bubbuls().unwrap(), before_bubbuls, "unrelated category untouched by the other two unlocks");
}

#[test]
fn mass_unlock_survives_write_and_reload() {
    let mut save = load_totk();
    save.unlock_all_bubbuls().unwrap();
    let bytes = save.to_bytes();

    // A grown file must still pass the version/size range check and load cleanly.
    let reloaded = save_engine::Save::detect(bytes).expect("grown save should still load");
    let save_engine::Save::Totk(reloaded) = reloaded else { panic!("fixture should be TOTK") };
    assert_eq!(reloaded.defeated_bubbuls().unwrap(), 147);
    assert!(!reloaded.pouch_weapons().unwrap().is_empty(), "other fields still readable after reload");
}

#[test]
fn mass_unlock_completes_within_a_generous_time_budget() {
    let mut save = load_totk();
    let start = std::time::Instant::now();
    save.unlock_all_bubbuls().unwrap();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 200,
        "mass-unlock took {elapsed:?}, expected well under 200ms for a single batched insertion"
    );
}

#[test]
fn teleporters_read_real_fixture_contents() {
    let save = load_totk();
    let teleporters = save.teleporters().unwrap();
    assert_eq!(teleporters.len(), 3);
    assert_eq!(teleporters[0].index, 1);
    assert!((teleporters[0].pos.0 - 3075.8567).abs() < 0.01);
}

#[test]
fn set_teleporters_round_trips_and_rejects_wrong_entry_count() {
    let mut save = load_totk();
    let mut teleporters = save.teleporters().unwrap();
    teleporters[0].pos = (1.0, 2.0, 3.0);
    teleporters[0].rot = (4.0, 5.0, 6.0);
    save.set_teleporters(&teleporters).unwrap();
    let reloaded = save.teleporters().unwrap();
    assert_eq!(reloaded[0].pos, (1.0, 2.0, 3.0));
    assert_eq!(reloaded[0].rot, (4.0, 5.0, 6.0));

    teleporters.pop();
    let result = save.set_teleporters(&teleporters);
    assert!(matches!(result, Err(save_engine::SaveError::SizeMismatch { .. })));
}

#[test]
fn browse_hashes_covers_the_real_fixtures_full_table() {
    let save = load_totk();
    let rows = save.browse_hashes().unwrap();
    assert_eq!(rows.len(), 30731);
    assert_eq!(rows.iter().filter(|r| r.editable).count(), 28920);
    assert_eq!(rows.iter().filter(|r| r.name.is_none()).count(), 32);
}

#[test]
fn set_hash_field_edits_a_scalar_field_found_by_hash() {
    let mut save = load_totk();
    let hash = save_engine::totk::murmur3::hash32("PlayerStatus.MaxLife");
    save.set_hash_field(hash, 777.0).unwrap();
    assert_eq!(save.max_life().unwrap(), 777);
}
