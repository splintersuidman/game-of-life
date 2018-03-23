#![feature(test)]

extern crate game_of_life;
extern crate test;

use game_of_life::GameOfLife;
use test::Bencher;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CHANCE: u8 = 128;

#[bench]
fn bench_new(b: &mut Bencher) {
    b.iter(|| GameOfLife::new(WIDTH, HEIGHT));
}

#[bench]
fn bench_to_string(b: &mut Bencher) {
    b.iter(|| GameOfLife::new(WIDTH, HEIGHT).to_string_with_alive('*'));
}

#[bench]
fn bench_init_empty(b: &mut Bencher) {
    b.iter(|| {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);
        gol.init_empty();
    });
}

#[bench]
fn bench_init_randomly(b: &mut Bencher) {
    b.iter(|| {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);
        gol.init_randomly(CHANCE);
    })
}

#[bench]
fn bench_update(b: &mut Bencher) {
    b.iter(|| {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);
        gol.init_randomly(CHANCE);
        gol.update();
    });
}
