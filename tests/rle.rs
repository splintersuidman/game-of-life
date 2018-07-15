extern crate game_of_life;
use game_of_life::file::{Parse, Serialise, RLE};

#[test]
fn test_rle_correct_file() {
    let input = "#N Pulsar
#O John Conway
#C A period 3 oscillator. Despite its size, this is the fourth most common oscillator (and by
#C far the most common of period greater than 2).
#C www.conwaylife.com/wiki/index.php?title=Pulsar
x = 13, y = 13, rule = B3/S23
2b3o3b3o2b2$o4bobo4bo$o4bobo4bo$o4bobo4bo$2b3o3b3o2b2$2b3o3b3o2b$o4bob
o4bo$o4bobo4bo$o4bobo4bo2$2b3o3b3o!";
    assert!(RLE::parse(&input).is_ok())
}

#[test]
fn test_rle_incorrect_file() {
    let input = "#N Pulsar
#O John Conway
#C A period 3 oscillator. Despite its size, this is the fourth most common oscillator (and by
#C far the most common of period greater than 2).
#C www.conwaylife.com/wiki/index.php?title=Pulsar
x = 13, y = 13, rule = B3/S23
2b3o3b3o2b2$o4bobo4bo$o4wrong4bo$o4bobo4bo$2b3o3b3o2b2$2b3o3b3o2b$o4bob
o4bo$o4bobo4bo$o4bobo4bo2$2b3o3b3o!";
    assert!(RLE::parse(&input).is_err())
}

#[test]
fn test_rle_serialise() {
    let input = "#C This is a glider.
x = 3, y = 3, rule = B3/S23
bo$2bo$3o!";
    let pattern = RLE::parse(&input).unwrap();
    let mut output = String::new();
    RLE::serialise(&mut output, pattern).unwrap();

    assert_eq!(&output, input)
}
