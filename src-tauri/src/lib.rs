pub mod commands;
pub mod dto;
pub mod error;
pub mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::botw::get_botw_state,
            commands::botw::set_rupees,
            commands::botw::set_mons,
            commands::botw::set_max_hearts,
            commands::botw::set_max_stamina,
            commands::botw::set_relic_gerudo,
            commands::botw::set_relic_goron,
            commands::botw::set_relic_rito,
            commands::botw::set_korok_seed_counter,
            commands::botw::set_defeated_hinox_counter,
            commands::botw::set_defeated_talus_counter,
            commands::botw::set_defeated_molduga_counter,
            commands::botw::set_playtime_seconds,
            commands::botw::set_motorcycle,
            commands::botw::set_player_position,
            commands::botw::set_horse_position,
            commands::botw::set_map,
            commands::botw::set_map_type,
            commands::botw::set_item,
            commands::botw::remove_item,
            commands::botw::duplicate_item,
            commands::botw::set_modifier,
            commands::botw::set_horse_name,
            commands::botw::set_horse_saddle,
            commands::botw::set_horse_reins,
            commands::botw::set_horse_type,
            commands::botw::unlock_all_koroks,
            commands::botw::unlock_all_defeated_hinox,
            commands::botw::unlock_all_defeated_talus,
            commands::botw::unlock_all_defeated_molduga,
            commands::botw::unlock_all_locations,
            commands::botw::set_botw_completism,
            commands::totk::get_totk_state,
            commands::totk::set_max_life,
            commands::totk::set_current_rupees,
            commands::totk::totk_set_max_stamina,
            commands::totk::set_max_energy,
            commands::totk::set_playtime,
            commands::totk::set_horse_inn_member_point,
            commands::totk::set_save_pos,
            commands::totk::set_sequence_current_banc,
            commands::totk::set_pouch_weapon_valid_num,
            commands::totk::set_pouch_bow_valid_num,
            commands::totk::set_pouch_shield_valid_num,
            commands::totk::set_pouch_weapons,
            commands::totk::set_pouch_bows,
            commands::totk::set_pouch_shields,
            commands::totk::set_armor,
            commands::totk::set_arrows,
            commands::totk::set_materials,
            commands::totk::set_key_items,
            commands::totk::set_devices,
            commands::totk::set_food,
            commands::totk::set_horses,
            commands::totk::set_autobuilds,
            commands::totk::set_map_pins,
            commands::totk::set_map_markers,
            commands::totk::set_teleporters,
            commands::totk::unlock_all_bubbuls,
            commands::totk::unlock_all_sage_wills,
            commands::totk::unlock_all_addison,
            commands::totk::set_totk_completism,
            commands::totk::get_totk_hash_rows,
            commands::totk::set_totk_hash_field,
            commands::totk::get_totk_abilities,
            commands::totk::set_totk_ability,
            commands::file::open_save,
            commands::file::open_save_at,
            commands::file::current_path,
            commands::file::save,
            commands::file::save_as,
            commands::local::discover_local_saves,
            commands::local::open_archive_save,
            commands::local::export_archive_copy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
