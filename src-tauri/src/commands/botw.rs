use tauri::State;

use crate::dto::{BotwState, ModifierCategoryDto};
use crate::error::ShellError;
use crate::state::AppState;
use save_engine::botw::BotwSave;
use save_engine::{Save, SaveError};

/// Builds the DTO from a loaded `BotwSave`. Pure and Tauri-free, directly unit-testable
/// against a real fixture.
pub fn read_state(save: &BotwSave) -> Result<BotwState, ShellError> {
    let (weapon_modifiers, bow_modifiers, shield_modifiers) = save.modifiers()?;
    Ok(BotwState {
        version: save_engine::botw::versions::VERSION[save.version_index].to_string(),
        modded: save.modded,
        rupees: save.rupees()?,
        mons: save.mons()?,
        max_hearts: save.max_hearts()?,
        max_stamina: save.max_stamina()?,
        relic_gerudo: save.relic_gerudo()?,
        relic_goron: save.relic_goron()?,
        relic_rito: save.relic_rito()?,
        korok_seed_counter: save.korok_seed_counter()?,
        defeated_hinox_counter: save.defeated_hinox_counter()?,
        defeated_talus_counter: save.defeated_talus_counter()?,
        defeated_molduga_counter: save.defeated_molduga_counter()?,
        playtime_seconds: save.playtime_seconds()?,
        motorcycle: save.motorcycle(),
        player_position: save.player_position()?,
        horse_position: save.horse_position()?,
        map: save.map()?,
        map_type: save.map_type()?,
        items: save.items_with_category()?.into_iter().map(crate::dto::BotwItemDto::from).collect(),
        weapon_modifiers: weapon_modifiers.into_iter().map(crate::dto::ItemModifierDto::from).collect(),
        bow_modifiers: bow_modifiers.into_iter().map(crate::dto::ItemModifierDto::from).collect(),
        shield_modifiers: shield_modifiers.into_iter().map(crate::dto::ItemModifierDto::from).collect(),
        horses: save.horses()?.into_iter().map(crate::dto::BotwHorseDto::from).collect(),
        completism: save.completism()?.into_iter().map(crate::dto::CompletismCategoryDto::from).collect(),
    })
}

pub fn get_botw_state_impl(app_state: &AppState) -> Result<BotwState, ShellError> {
    let guard = crate::state::lock(&app_state.save);
    match guard.as_ref() {
        Some(Save::Botw(save)) => read_state(save),
        Some(Save::Totk(_)) => Err(ShellError::wrong_game("BOTW")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn get_botw_state(state: State<'_, AppState>) -> Result<BotwState, ShellError> {
    get_botw_state_impl(state.inner())
}

/// Shared plumbing for every BOTW setter: lock the state, confirm a BOTW save is loaded,
/// run `f` against it, map any engine error through `ShellError`. Pure and Tauri-free.
pub fn with_botw<T>(
    app_state: &AppState,
    f: impl FnOnce(&mut BotwSave) -> Result<T, SaveError>,
) -> Result<T, ShellError> {
    let mut guard = crate::state::lock(&app_state.save);
    match guard.as_mut() {
        Some(Save::Botw(save)) => f(save).map_err(ShellError::from),
        Some(Save::Totk(_)) => Err(ShellError::wrong_game("BOTW")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn set_rupees(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_rupees(val))
}

#[tauri::command]
pub fn set_mons(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_mons(val))
}

#[tauri::command]
pub fn set_max_hearts(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_max_hearts(val))
}

#[tauri::command]
pub fn set_max_stamina(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_max_stamina(val))
}

#[tauri::command]
pub fn set_relic_gerudo(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_relic_gerudo(val))
}

#[tauri::command]
pub fn set_relic_goron(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_relic_goron(val))
}

#[tauri::command]
pub fn set_relic_rito(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_relic_rito(val))
}

#[tauri::command]
pub fn set_korok_seed_counter(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_korok_seed_counter(val))
}

#[tauri::command]
pub fn set_defeated_hinox_counter(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_defeated_hinox_counter(val))
}

#[tauri::command]
pub fn set_defeated_talus_counter(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_defeated_talus_counter(val))
}

#[tauri::command]
pub fn set_defeated_molduga_counter(
    state: State<'_, AppState>,
    val: u32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_defeated_molduga_counter(val))
}

#[tauri::command]
pub fn set_playtime_seconds(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_playtime_seconds(val))
}

#[tauri::command]
pub fn set_motorcycle(state: State<'_, AppState>, val: bool) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| {
        save.set_motorcycle(val);
        Ok(())
    })
}

#[tauri::command]
pub fn set_player_position(
    state: State<'_, AppState>,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_player_position(x, y, z))
}

#[tauri::command]
pub fn set_horse_position(
    state: State<'_, AppState>,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_horse_position(x, y, z))
}

#[tauri::command]
pub fn set_map(state: State<'_, AppState>, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_map(&value))
}

#[tauri::command]
pub fn set_map_type(state: State<'_, AppState>, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_map_type(&value))
}

#[tauri::command]
pub fn set_item(
    state: State<'_, AppState>,
    index: usize,
    name: String,
    quantity: u32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_item(index, &name, quantity))
}

#[tauri::command]
pub fn remove_item(state: State<'_, AppState>, index: usize) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.remove_item(index))
}

#[tauri::command]
pub fn duplicate_item(state: State<'_, AppState>, index: usize) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.duplicate_item(index))
}

#[tauri::command]
pub fn set_modifier(
    state: State<'_, AppState>,
    category: ModifierCategoryDto,
    index: usize,
    modifier: u32,
    value: u32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_modifier(category.into(), index, modifier, value))
}

#[tauri::command]
pub fn set_horse_name(state: State<'_, AppState>, index: usize, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_horse_name(index, &value))
}

#[tauri::command]
pub fn set_horse_saddle(state: State<'_, AppState>, index: usize, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_horse_saddle(index, &value))
}

#[tauri::command]
pub fn set_horse_reins(state: State<'_, AppState>, index: usize, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_horse_reins(index, &value))
}

#[tauri::command]
pub fn set_horse_type(state: State<'_, AppState>, index: usize, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_horse_type(index, &value))
}

#[tauri::command]
pub fn unlock_all_koroks(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_botw(state.inner(), |save| save.unlock_all_koroks())
}

#[tauri::command]
pub fn unlock_all_defeated_hinox(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_botw(state.inner(), |save| save.unlock_all_defeated_hinox())
}

#[tauri::command]
pub fn unlock_all_defeated_talus(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_botw(state.inner(), |save| save.unlock_all_defeated_talus())
}

#[tauri::command]
pub fn unlock_all_defeated_molduga(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_botw(state.inner(), |save| save.unlock_all_defeated_molduga())
}

#[tauri::command]
pub fn unlock_all_locations(state: State<'_, AppState>) -> Result<usize, ShellError> {
    with_botw(state.inner(), |save| save.unlock_all_locations())
}

/// Mass-completes one BOTW completionism category by id (from the state's `completism` list).
/// `metric` is accepted for symmetry with TOTK but unused (every BOTW card has one metric).
#[tauri::command]
pub fn set_botw_completism(state: State<'_, AppState>, id: String, metric: usize) -> Result<usize, ShellError> {
    with_botw(state.inner(), |save| save.set_completism(&id, metric))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_bytes() -> Vec<u8> {
        std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present")
    }

    fn loaded_app_state() -> AppState {
        let save = Save::detect(fixture_bytes()).expect("should detect a known BOTW save");
        AppState { save: std::sync::Mutex::new(Some(save)), ..AppState::default() }
    }

    #[test]
    fn get_botw_state_impl_reads_real_fixture_values() {
        let app_state = loaded_app_state();
        let dto = get_botw_state_impl(&app_state).unwrap();

        // Cross-check against a second, independently loaded instance of the same
        // fixture calling the engine's own accessor directly, a real assertion that
        // the DTO faithfully mirrors `save-engine`, not just that the call didn't panic.
        let expected_rupees = match Save::detect(fixture_bytes()).unwrap() {
            Save::Botw(save) => save.rupees().unwrap(),
            Save::Totk(_) => panic!("fixture should be BOTW"),
        };
        assert_eq!(dto.rupees, expected_rupees);
        assert!(!dto.map.is_empty());
    }

    #[test]
    fn get_botw_state_impl_on_totk_save_returns_wrong_game() {
        let totk_bytes = std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present");
        let save = Save::detect(totk_bytes).expect("should detect a known TOTK save");
        let app_state = AppState { save: std::sync::Mutex::new(Some(save)), ..AppState::default() };
        let result = get_botw_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "wrong_game");
    }

    #[test]
    fn get_botw_state_impl_with_no_save_returns_no_save_loaded() {
        let app_state = AppState::default();
        let result = get_botw_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "no_save_loaded");
    }

    #[test]
    fn with_botw_edits_rupees_and_persists_in_state() {
        let app_state = loaded_app_state();
        let before = get_botw_state_impl(&app_state).unwrap().rupees;
        with_botw(&app_state, |save| save.set_rupees(before.wrapping_add(500))).unwrap();
        let after = get_botw_state_impl(&app_state).unwrap();
        assert_eq!(after.rupees, before.wrapping_add(500));
    }

    #[test]
    fn with_botw_on_totk_save_returns_wrong_game() {
        let totk_bytes = std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present");
        let save = Save::detect(totk_bytes).expect("should detect a known TOTK save");
        let app_state = AppState { save: std::sync::Mutex::new(Some(save)), ..AppState::default() };
        let result = with_botw(&app_state, |save| save.set_rupees(1));
        assert_eq!(result.unwrap_err().kind, "wrong_game");
    }

    #[test]
    fn with_botw_edits_motorcycle_and_persists_in_state() {
        let app_state = loaded_app_state();
        let initial_motorcycle = get_botw_state_impl(&app_state).unwrap().motorcycle;

        // If the fixture has the motorcycle field, setting it should persist; if not, it stays None.
        with_botw(&app_state, |save| {
            save.set_motorcycle(true);
            Ok(())
        })
        .unwrap();

        let after_motorcycle = get_botw_state_impl(&app_state).unwrap().motorcycle;
        // If the field exists in the save version, it should be updated to true.
        // If not (None initially), it should remain None.
        if initial_motorcycle.is_some() {
            assert_eq!(after_motorcycle, Some(true));
        } else {
            assert_eq!(after_motorcycle, None);
        }
    }

    #[test]
    fn get_botw_state_impl_includes_items_modifiers_and_horses() {
        let app_state = loaded_app_state();
        let dto = get_botw_state_impl(&app_state).unwrap();
        assert!(!dto.items.is_empty());
        assert_eq!(dto.horses.len(), save_engine::botw::NUM_HORSE_SLOTS);
    }

    #[test]
    fn set_item_round_trips_through_with_botw() {
        let app_state = loaded_app_state();
        let items_before = get_botw_state_impl(&app_state).unwrap().items;
        let last_index = items_before.len() - 1;
        with_botw(&app_state, |save| save.set_item(last_index, "Weapon_Sword_070", 1)).unwrap();
        let items_after = get_botw_state_impl(&app_state).unwrap().items;
        assert_eq!(items_after[last_index].name, "Weapon_Sword_070");
        assert_eq!(items_after[last_index].quantity, 1);
    }

    #[test]
    fn set_horse_name_round_trips_through_with_botw() {
        let app_state = loaded_app_state();
        with_botw(&app_state, |save| save.set_horse_name(0, "Epona")).unwrap();
        let horses = get_botw_state_impl(&app_state).unwrap().horses;
        assert_eq!(horses[0].name, Some("Epona".to_string()));
    }

    #[test]
    fn get_botw_state_impl_includes_item_categories() {
        let app_state = loaded_app_state();
        let dto = get_botw_state_impl(&app_state).unwrap();
        // The real fixture's item list mixes categories; this just confirms the DTO
        // carries a real category rather than always defaulting to KeyItem.
        assert!(dto
            .items
            .iter()
            .any(|item| item.category != crate::dto::ItemCategoryDto::KeyItem));
    }

    #[test]
    fn unlock_all_koroks_returns_newly_found_count_and_persists() {
        let app_state = loaded_app_state();
        let before = get_botw_state_impl(&app_state).unwrap().korok_seed_counter;
        let unlocked = with_botw(&app_state, |save| save.unlock_all_koroks()).unwrap();
        let after = get_botw_state_impl(&app_state).unwrap().korok_seed_counter;
        assert_eq!(after, before + unlocked as u32);

        // Idempotent: running it again finds nothing new.
        let second = with_botw(&app_state, |save| save.unlock_all_koroks()).unwrap();
        assert_eq!(second, 0);
    }

    #[test]
    fn unlock_all_locations_returns_newly_visited_count() {
        let app_state = loaded_app_state();
        let unlocked = with_botw(&app_state, |save| save.unlock_all_locations()).unwrap();
        let second = with_botw(&app_state, |save| save.unlock_all_locations()).unwrap();
        assert_eq!(second, 0, "should be idempotent");
        assert!(unlocked > 0, "fixture should have at least one unvisited location");
    }
}
