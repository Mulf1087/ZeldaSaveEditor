//! Opt-in regression against a user's own saves. Set ZELDA_TEST_SAVE_LIST to
//! a newline-separated UTF-8 path list; source files are never written.

use save_engine::Save;

#[test]
fn local_saves_parse_and_round_trip_without_touching_sources() {
    let Ok(list_path) = std::env::var("ZELDA_TEST_SAVE_LIST") else { return };
    let paths = std::fs::read_to_string(list_path).expect("save list exists");
    for path in paths.lines().filter(|path| !path.is_empty()) {
        let original = std::fs::read(path).expect("local save exists");
        let save = Save::detect(original.clone()).expect("local save format is supported");
        let serialized = save.to_bytes();
        assert_eq!(serialized, original, "no-edit round trip changed {path}");

        let mut save = Save::detect(original.clone()).expect("local save can be reloaded");
        let expected = match &mut save {
            Save::Botw(botw) => {
                let value = botw.rupees().unwrap();
                let changed = if value == u32::MAX { value - 1 } else { value + 1 };
                botw.set_rupees(changed).unwrap();
                changed
            }
            Save::Totk(totk) => {
                let value = totk.current_rupees().unwrap();
                let changed = if value == u32::MAX { value - 1 } else { value + 1 };
                totk.set_current_rupees(changed).unwrap();
                changed
            }
        };
        let edited = save.to_bytes();
        assert_ne!(edited, original, "edit had no effect for {path}");
        let reloaded = Save::detect(edited).expect("edited save is parseable");
        let actual = match reloaded {
            Save::Botw(botw) => botw.rupees().unwrap(),
            Save::Totk(totk) => totk.current_rupees().unwrap(),
        };
        assert_eq!(actual, expected, "rupee change did not round-trip for {path}");
        assert_eq!(std::fs::read(path).unwrap(), original, "test modified source {path}");
    }
}
