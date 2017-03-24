use highscores;
use highscores::Highscores;
use serde_json;
use std::fs;
use std::fs::File;

#[test]
fn highscores_test_serialize() {

    let mut hs = Highscores::new();
    hs.set_beginner(10230);
    hs.set_intermediate(23864);

    let expected_json = "{\"beginner\":10230,\"intermediate\":23864,\"expert\":null}";

    let actual_json = serde_json::to_string(&hs).unwrap();

    assert_eq!(expected_json, actual_json);
}

#[test]
fn highscores_test_deserialize() {

    let json = "{\"beginner\":null,\"intermediate\":1000,\"expert\":666666}";

    let hs: Highscores = serde_json::from_str(json).unwrap();

    assert_eq!(hs.get_beginner(), Option::None);
    assert_eq!(hs.get_intermediate(), Option::Some(1000 as u64));
    assert_eq!(hs.get_expert(), Option::Some(666666));
}

#[test]
fn highscores_test_save() {

    let mut hs = Highscores::new();
    hs.set_beginner(10230);
    hs.set_intermediate(23864);

    highscores::save(&hs, "test.txt");

    let loaded_highscores = highscores::load("test.txt");

    assert_eq!(hs, loaded_highscores);
}

#[test]
fn create_new_file() {

    let mut hs = Highscores::new();
    hs.set_beginner(10230);
    hs.set_intermediate(23864);

    highscores::save(&hs, "hs.json");

    fs::remove_file("hs.json").unwrap();
}

#[test]
fn load_without_file() {

    let loaded_highscores = highscores::load("hs.json");

    assert_eq!(loaded_highscores, Highscores::new());
}

#[test]
fn load_unreadable_file() {

    File::create("hs.json").unwrap();

    let loaded_highscores = highscores::load("hs.json");

    assert_eq!(loaded_highscores, Highscores::new());

    fs::remove_file("hs.json").unwrap();
}