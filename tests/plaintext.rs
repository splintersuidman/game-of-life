extern crate game_of_life;
use game_of_life::file::{Parse, Plaintext, Serialise};

#[test]
fn test_plaintext_is_plaintext_file() {
    assert!(Plaintext::is_type(&"!Name: My name"));
    assert!(!Plaintext::is_type(&"No name"));
}

#[test]
fn test_plaintext_correct_file() {
    let file = "!Name: My name\n.O\n..O\nOOO";
    assert!(Plaintext::parse(&file).is_ok())
}

#[test]
fn test_plaintext_incorrect_file() {
    let file = "!Name: My name\n.O\n..Owrong characters\nOOO";
    assert!(Plaintext::parse(&file).is_err())
}

#[test]
fn test_plaintext_serialise() {
    let input = "!Name: Glider
!Author: Someone
!This is a glider.
!It was discovered early on and is frequently seen.
.O.
..O
OOO";
    let pattern = Plaintext::parse(&input).unwrap();
    let mut output = String::new();
    Plaintext::serialise(&mut output, pattern).unwrap();

    assert_eq!(&output, input)
}
