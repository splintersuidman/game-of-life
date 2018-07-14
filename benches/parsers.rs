#![cfg_attr(nightly, feature(bench))]
#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {

    extern crate game_of_life;
    extern crate test;

    use self::game_of_life::file::*;
    use self::test::Bencher;

    const LIDKA_PLAINTEXT: &str = include_str!("../examples/lidka.cells");
    const CORDERSHIP_GUN: &str = include_str!("../examples/3enginecordershipgun_106.lif");
    const BOMBER: &str = include_str!("../examples/B-52_Bomber_105.life");
    const PULSAR: &str = include_str!("../examples/Pulsar.rle");

    #[bench]
    fn bench_parse_plaintext(b: &mut Bencher) {
        b.iter(|| Plaintext::parse(&LIDKA_PLAINTEXT));
    }

    #[bench]
    fn bench_parse_life_106(b: &mut Bencher) {
        b.iter(|| Life106::parse(&CORDERSHIP_GUN))
    }

    #[bench]
    fn bench_parse_life_105(b: &mut Bencher) {
        b.iter(|| Life105::parse(&BOMBER));
    }

    #[bench]
    fn bench_parse_rle(b: &mut Bencher) {
        b.iter(|| RLE::parse(&PULSAR));
    }
}
