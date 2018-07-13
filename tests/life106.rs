extern crate game_of_life;
use game_of_life::file::{Life106, Parse};

#[test]
fn test_life_106_is_life_106_file() {
    assert!(Life106::is_type(&"#Life 1.06\n5 0"));
    assert!(!Life106::is_type(&"#Life 1.05\n5 0"));
}

#[test]
fn test_life_106_correct_file() {
    let file = "#Life 1.06\n-5 0\n6 7";
    assert!(Life106::parse(&file).is_ok())
}

#[test]
fn test_life_106_incorrect_file() {
    let file = "#Life 1.06\n-a b\nc d";
    assert!(Life106::parse(&file).is_err());
    let file = "#Life 1.06\na b\nc d";
    assert!(Life106::parse(&file).is_err());
}
