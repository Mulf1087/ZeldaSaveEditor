use tauri::State;

use crate::dto::{
    TotkAbilityDto, TotkArmorDto, TotkArrowDto, TotkAutoBuildEntryDto, TotkBowDto, TotkDeviceDto,
    TotkFoodDto, TotkHashRowDto, TotkHorseDto, TotkKeyItemDto, TotkMapMarkerDto, TotkMapPinDto,
    TotkMaterialDto, TotkShieldDto, TotkState, TotkTeleporterDto, TotkWeaponDto,
};
use crate::error::ShellError;
use crate::state::AppState;
use save_engine::totk::TotkSave;
use save_engine::{Save, SaveError};

pub fn read_state(save: &TotkSave) -> Result<TotkState, ShellError> {
    Ok(TotkState {
        version: save.version_label.to_string(),
        modded: save.modded,
        max_life: save.max_life()?,
        current_rupees: save.current_rupees()?,
        max_stamina: save.max_stamina()?,
        max_energy: save.max_energy()?,
        playtime: save.playtime()?,
        horse_inn_member_point: save.horse_inn_member_point()?,
        save_pos: save.save_pos()?,
        sequence_current_banc: save.sequence_current_banc()?,
        pouch_weapon_valid_num: save.pouch_weapon_valid_num()?,
        pouch_bow_valid_num: save.pouch_bow_valid_num()?,
        pouch_shield_valid_num: save.pouch_shield_valid_num()?,
        pouch_weapons: save.pouch_weapons()?.into_iter().map(TotkWeaponDto::from).collect(),
        pouch_bows: save.pouch_bows()?.into_iter().map(TotkBowDto::from).collect(),
        pouch_shields: save.pouch_shields()?.into_iter().map(TotkShieldDto::from).collect(),
        armor: save.armor()?.into_iter().map(TotkArmorDto::from).collect(),
        arrows: save.arrows()?.into_iter().map(TotkArrowDto::from).collect(),
        materials: save.materials()?.into_iter().map(TotkMaterialDto::from).collect(),
        key_items: save.key_items()?.into_iter().map(TotkKeyItemDto::from).collect(),
        devices: save.devices()?.into_iter().map(TotkDeviceDto::from).collect(),
        food: save.food()?.into_iter().map(TotkFoodDto::from).collect(),
        horses: save.horses()?.into_iter().map(TotkHorseDto::from).collect(),
        completism: save.completism()?.into_iter().map(crate::dto::CompletismCategoryDto::from).collect(),
        autobuilds: save.autobuilds()?.into_iter().map(TotkAutoBuildEntryDto::from).collect(),
        map_pins: save.map_pins()?.into_iter().map(TotkMapPinDto::from).collect(),
        map_markers: save.map_markers()?.into_iter().map(TotkMapMarkerDto::from).collect(),
        teleporters: save.teleporters()?.into_iter().map(TotkTeleporterDto::from).collect(),
    })
}

pub fn get_totk_state_impl(app_state: &AppState) -> Result<TotkState, ShellError> {
    let guard = crate::state::lock(&app_state.save);
    match guard.as_ref() {
        Some(Save::Totk(save)) => read_state(save),
        Some(Save::Botw(_)) => Err(ShellError::wrong_game("TOTK")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn get_totk_state(state: State<'_, AppState>) -> Result<TotkState, ShellError> {
    get_totk_state_impl(state.inner())
}

pub fn with_totk<T>(
    app_state: &AppState,
    f: impl FnOnce(&mut TotkSave) -> Result<T, SaveError>,
) -> Result<T, ShellError> {
    let mut guard = crate::state::lock(&app_state.save);
    match guard.as_mut() {
        Some(Save::Totk(save)) => f(save).map_err(ShellError::from),
        Some(Save::Botw(_)) => Err(ShellError::wrong_game("TOTK")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn set_max_life(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_max_life(val))
}

#[tauri::command]
pub fn set_current_rupees(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_current_rupees(val))
}

// Tauri v2's #[tauri::command] macro generates crate-level symbols per literal function
// name, so a TOTK command sharing a name with a BOTW command collides at compile time
// (E0428). Only this one name (`set_max_stamina`) actually collides with anything in
// commands::botw. Every other TOTK setter keeps its plain name matching TotkSave's own
// method name; renamed only this one instead of prefixing the whole file.
#[tauri::command]
pub fn totk_set_max_stamina(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_max_stamina(val))
}

#[tauri::command]
pub fn set_max_energy(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_max_energy(val))
}

#[tauri::command]
pub fn set_playtime(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_playtime(val))
}

#[tauri::command]
pub fn set_horse_inn_member_point(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_horse_inn_member_point(val))
}

#[tauri::command]
pub fn set_save_pos(state: State<'_, AppState>, x: f32, y: f32, z: f32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_save_pos(x, y, z))
}

#[tauri::command]
pub fn set_sequence_current_banc(state: State<'_, AppState>, value: String) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_sequence_current_banc(&value))
}

#[tauri::command]
pub fn set_pouch_weapon_valid_num(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_pouch_weapon_valid_num(val))
}

#[tauri::command]
pub fn set_pouch_bow_valid_num(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_pouch_bow_valid_num(val))
}

#[tauri::command]
pub fn set_pouch_shield_valid_num(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_pouch_shield_valid_num(val))
}

#[tauri::command]
pub fn set_pouch_weapons(state: State<'_, AppState>, entries: Vec<TotkWeaponDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_pouch_weapons(&entries))
}

#[tauri::command]
pub fn set_pouch_bows(state: State<'_, AppState>, entries: Vec<TotkBowDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_pouch_bows(&entries))
}

#[tauri::command]
pub fn set_pouch_shields(state: State<'_, AppState>, entries: Vec<TotkShieldDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_pouch_shields(&entries))
}

#[tauri::command]
pub fn set_armor(state: State<'_, AppState>, entries: Vec<TotkArmorDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_armor(&entries))
}

#[tauri::command]
pub fn set_arrows(state: State<'_, AppState>, entries: Vec<TotkArrowDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_arrows(&entries))
}

#[tauri::command]
pub fn set_materials(state: State<'_, AppState>, entries: Vec<TotkMaterialDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_materials(&entries))
}

#[tauri::command]
pub fn set_key_items(state: State<'_, AppState>, entries: Vec<TotkKeyItemDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_key_items(&entries))
}

#[tauri::command]
pub fn set_devices(state: State<'_, AppState>, entries: Vec<TotkDeviceDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_devices(&entries))
}

#[tauri::command]
pub fn set_food(state: State<'_, AppState>, entries: Vec<TotkFoodDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_food(&entries))
}

#[tauri::command]
pub fn set_horses(state: State<'_, AppState>, entries: Vec<TotkHorseDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_horses(&entries))
}

#[tauri::command]
pub fn set_autobuilds(state: State<'_, AppState>, entries: Vec<TotkAutoBuildEntryDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_autobuilds(&entries))
}

#[tauri::command]
pub fn set_map_pins(state: State<'_, AppState>, entries: Vec<TotkMapPinDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_map_pins(&entries))
}

#[tauri::command]
pub fn set_map_markers(state: State<'_, AppState>, entries: Vec<TotkMapMarkerDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_map_markers(&entries))
}

#[tauri::command]
pub fn set_teleporters(state: State<'_, AppState>, entries: Vec<TotkTeleporterDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_teleporters(&entries))
}

#[tauri::command]
pub fn unlock_all_bubbuls(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_totk(state.inner(), |save| save.unlock_all_bubbuls())
}

#[tauri::command]
pub fn unlock_all_sage_wills(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_totk(state.inner(), |save| save.unlock_all_sage_wills())
}

#[tauri::command]
pub fn unlock_all_addison(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_totk(state.inner(), |save| save.unlock_all_addison())
}

/// Sets one completionism metric to complete. `id` is a category id from the state's `completism`
/// list; `metric` is that card's metric index (0 = primary, 1 = the secondary line on the
/// towers/shrines/lightroots cards). Returns how many entries changed.
#[tauri::command]
pub fn set_totk_completism(state: State<'_, AppState>, id: String, metric: usize) -> Result<usize, ShellError> {
    with_totk(state.inner(), |save| save.set_completism(&id, metric))
}

// --- Advanced: generic hash browser. Kept out of `TotkState`/`read_state`: the full table is
// ~30k rows, and every edit already triggers a `get_totk_state` refresh, so folding this in
// would send that whole payload on every single unrelated edit. Fetched on demand instead. ---

pub fn get_totk_hash_rows_impl(app_state: &AppState) -> Result<Vec<TotkHashRowDto>, ShellError> {
    let guard = crate::state::lock(&app_state.save);
    match guard.as_ref() {
        Some(Save::Totk(save)) => Ok(save.browse_hashes()?.into_iter().map(TotkHashRowDto::from).collect()),
        Some(Save::Botw(_)) => Err(ShellError::wrong_game("TOTK")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn get_totk_hash_rows(state: State<'_, AppState>) -> Result<Vec<TotkHashRowDto>, ShellError> {
    get_totk_hash_rows_impl(state.inner())
}

#[tauri::command]
pub fn set_totk_hash_field(state: State<'_, AppState>, hash: u32, value: f64) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_hash_field(hash, value))
}

// --- Key-item ability toggles ---

pub fn get_totk_abilities_impl(app_state: &AppState) -> Result<Vec<TotkAbilityDto>, ShellError> {
    let guard = crate::state::lock(&app_state.save);
    match guard.as_ref() {
        Some(Save::Totk(save)) => Ok(save.abilities()?.into_iter().map(TotkAbilityDto::from).collect()),
        Some(Save::Botw(_)) => Err(ShellError::wrong_game("TOTK")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn get_totk_abilities(state: State<'_, AppState>) -> Result<Vec<TotkAbilityDto>, ShellError> {
    get_totk_abilities_impl(state.inner())
}

#[tauri::command]
pub fn set_totk_ability(state: State<'_, AppState>, id: String, on: bool) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_ability(&id, on))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn totk_fixture_bytes() -> Vec<u8> {
        std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present")
    }

    fn botw_fixture_bytes() -> Vec<u8> {
        std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present")
    }

    fn loaded_app_state() -> AppState {
        let save = Save::detect(totk_fixture_bytes()).expect("should detect a known TOTK save");
        AppState { save: std::sync::Mutex::new(Some(save)), ..AppState::default() }
    }

    #[test]
    fn get_totk_state_impl_reads_real_fixture_values() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert!(dto.max_life > 0);
    }

    #[test]
    fn get_totk_state_impl_on_botw_save_returns_wrong_game() {
        let save = Save::detect(botw_fixture_bytes()).expect("should detect a known BOTW save");
        let app_state = AppState { save: std::sync::Mutex::new(Some(save)), ..AppState::default() };
        let result = get_totk_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "wrong_game");
    }

    #[test]
    fn get_totk_state_impl_with_no_save_returns_no_save_loaded() {
        let app_state = AppState::default();
        let result = get_totk_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "no_save_loaded");
    }

    #[test]
    fn with_totk_edits_current_rupees_and_persists_in_state() {
        let app_state = loaded_app_state();
        let before = get_totk_state_impl(&app_state).unwrap().current_rupees;
        with_totk(&app_state, |save| save.set_current_rupees(before.wrapping_add(500))).unwrap();
        let after = get_totk_state_impl(&app_state).unwrap();
        assert_eq!(after.current_rupees, before.wrapping_add(500));
    }

    #[test]
    fn with_totk_edits_save_pos_and_persists_in_state() {
        let app_state = loaded_app_state();
        with_totk(&app_state, |save| save.set_save_pos(1.0, 2.0, 3.0)).unwrap();
        let after = get_totk_state_impl(&app_state).unwrap();
        assert_eq!(after.save_pos, (1.0, 2.0, 3.0));
    }

    #[test]
    fn get_totk_state_impl_includes_pouch_and_horse_entries() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert!(!dto.pouch_weapons.is_empty());
        assert!(!dto.armor.is_empty());
        assert!(!dto.horses.is_empty());
        assert_eq!(dto.horses[0].name, "Max");
    }

    #[test]
    fn set_pouch_weapons_round_trips_through_with_totk() {
        let app_state = loaded_app_state();
        let mut weapons = get_totk_state_impl(&app_state).unwrap().pouch_weapons;
        weapons.push(crate::dto::TotkWeaponDto {
            id: "Weapon_Sword_001".into(),
            durability: 20,
            modifier: 0,
            modifier_value: 0,
            fuse_id: String::new(),
            fuse_durability: 0,
            extra_durability: 0,
            record_extra_durability: -1,
        });
        with_totk(&app_state, |save| {
            save.set_pouch_weapons(&weapons.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().pouch_weapons;
        assert_eq!(after.last().unwrap().id, "Weapon_Sword_001");
    }

    #[test]
    fn set_horses_round_trips_through_with_totk() {
        let app_state = loaded_app_state();
        let mut horses = get_totk_state_impl(&app_state).unwrap().horses;
        horses[0].name = "Renamed".to_string();
        with_totk(&app_state, |save| {
            save.set_horses(&horses.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().horses;
        assert_eq!(after[0].name, "Renamed");
    }

    #[test]
    fn get_totk_state_impl_includes_completism_counts() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        let shrines = dto.completism.iter().find(|c| c.id == "shrines").unwrap();
        assert_eq!(shrines.metrics[0].max, 152);
        // The whole card grid is present, including categories only added by the parity work.
        for id in ["towers", "lightroots", "caves", "wells", "chasms", "gleeok", "compendium"] {
            assert!(dto.completism.iter().any(|c| c.id == id), "missing category {id}");
        }
    }

    #[test]
    fn get_totk_state_impl_includes_autobuilds() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert!(!dto.autobuilds.is_empty());
        assert!(dto.autobuilds.iter().all(|e| e.combined_actor_info.len() == 6688));
    }

    #[test]
    fn set_autobuilds_round_trips_favorite_flag_through_with_totk() {
        let app_state = loaded_app_state();
        let mut autobuilds = get_totk_state_impl(&app_state).unwrap().autobuilds;
        let flipped = !autobuilds[0].is_favorite;
        autobuilds[0].is_favorite = flipped;
        with_totk(&app_state, |save| {
            save.set_autobuilds(&autobuilds.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().autobuilds;
        assert_eq!(after[0].is_favorite, flipped);
    }

    #[test]
    fn get_totk_state_impl_includes_map_pins() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert_eq!(dto.map_pins.len(), 300);
        assert!(dto.map_pins.iter().any(|p| p.icon != save_engine::totk::mapdata::ICON_NONE));
    }

    #[test]
    fn set_map_pins_round_trips_through_with_totk() {
        let app_state = loaded_app_state();
        let mut pins = get_totk_state_impl(&app_state).unwrap().map_pins;
        pins[0].icon = save_engine::totk::mapdata::ICON_NONE;
        with_totk(&app_state, |save| {
            save.set_map_pins(&pins.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().map_pins;
        assert_eq!(after[0].icon, save_engine::totk::mapdata::ICON_NONE);
    }

    #[test]
    fn get_totk_state_impl_includes_markers_and_teleporters() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert_eq!(dto.map_markers.len(), 6);
        assert_eq!(dto.teleporters.len(), 3);
    }

    #[test]
    fn set_teleporters_round_trips_through_with_totk() {
        let app_state = loaded_app_state();
        let mut teleporters = get_totk_state_impl(&app_state).unwrap().teleporters;
        teleporters[0].pos = (1.0, 2.0, 3.0);
        with_totk(&app_state, |save| {
            save.set_teleporters(&teleporters.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().teleporters;
        assert_eq!(after[0].pos, (1.0, 2.0, 3.0));
    }

    #[test]
    fn unlock_all_bubbuls_command_returns_newly_unlocked_count_and_persists() {
        let app_state = loaded_app_state();
        let bubbuls = |s: &crate::dto::TotkState| {
            s.completism.iter().find(|c| c.id == "bubbuls").unwrap().metrics[0].value
        };
        let before = bubbuls(&get_totk_state_impl(&app_state).unwrap());

        let count = with_totk(&app_state, |save| save.unlock_all_bubbuls()).unwrap();

        assert!(count > 0);
        let after = bubbuls(&get_totk_state_impl(&app_state).unwrap());
        assert_eq!(after, 147);
        assert!(after > before);
    }

    #[test]
    fn get_totk_hash_rows_returns_the_full_table_with_editable_and_unknown_rows() {
        let app_state = loaded_app_state();
        let rows = get_totk_hash_rows_impl(&app_state).unwrap();
        assert_eq!(rows.len(), 30731);
    }

    #[test]
    fn set_totk_hash_field_edits_a_known_scalar_field() {
        let app_state = loaded_app_state();
        let hash = save_engine::totk::murmur3::hash32("PlayerStatus.MaxLife");
        with_totk(&app_state, |save| save.set_hash_field(hash, 555.0)).unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().max_life;
        assert_eq!(after, 555);
    }
}
