extern crate game_of_life;
use game_of_life::Rule;

#[test]
fn test_rule_display_birth() {
    let normal = Rule::normal();
    let normal_birth = String::from("3");

    assert_eq!(normal.display_birth(), normal_birth);
}

#[test]
fn test_rule_display_survival() {
    let normal = Rule::normal();
    let normal_survival = String::from("23");

    assert_eq!(normal.display_survival(), normal_survival);
}
