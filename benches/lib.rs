#![cfg_attr(feature = "bench-gol", feature(test))]

#[cfg(all(feature = "bench-gol", test))]
mod benches {
    extern crate game_of_life;
    extern crate test;

    use self::game_of_life::GameOfLife;
    use self::test::Bencher;

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 100;
    const CHANCE: u8 = 128;
    const TEST_FILE: &str = "./examples/B-52_Bomber_105.life";

    #[bench]
    fn bench_new(b: &mut Bencher) {
        b.iter(|| GameOfLife::new(WIDTH, HEIGHT));
    }

    #[bench]
    fn bench_to_string(b: &mut Bencher) {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);
        gol.init_randomly(CHANCE);
        b.iter(|| {
            gol.to_string_with_alive('*');
        });
    }

    #[bench]
    fn bench_init_empty(b: &mut Bencher) {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);

        b.iter(|| {
            gol.init_empty();
        });
    }

    #[bench]
    fn bench_init_randomly(b: &mut Bencher) {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);

        b.iter(|| {
            gol.init_randomly(CHANCE);
        })
    }

    #[bench]
    fn bench_init_with_file(b: &mut Bencher) {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);

        b.iter(|| {
            gol.init_with_file(&TEST_FILE).unwrap();
        });
    }

    #[bench]
    fn bench_update(b: &mut Bencher) {
        let mut gol = GameOfLife::new(WIDTH, HEIGHT);
        gol.init_randomly(CHANCE);

        b.iter(|| {
            gol.update();
        });
    }
}
