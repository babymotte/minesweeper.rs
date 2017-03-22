use highscores::Highscores;
use serde_json;

#[test]
fn highscores_test_serialize() {

    let mut hs = Highscores::new();
    hs.set_beginner(10.23);
    hs.set_intermediate(23.864);

    let expected_json = "{\"beginner\":10.23,\"intermediate\":23.864,\"expert\":null}";

    let actual_json = serde_json::to_string(&hs).unwrap();

    assert_eq!(expected_json, actual_json);
}

#[test]
fn highscores_test_deserialize() {

    let json = "{\"beginner\":null,\"intermediate\":1,\"expert\":666.666}";

    let hs: Highscores = serde_json::from_str(json).unwrap();

    assert_eq!(hs.get_beginner(), Option::None);
    assert_eq!(hs.get_intermediate(), Option::Some(1 as f64));
    assert_eq!(hs.get_expert(), Option::Some(666.666));
}