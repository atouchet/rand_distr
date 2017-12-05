#![feature(test)]

extern crate test;
extern crate rand;

const RAND_BENCH_N: u64 = 1000;

mod distributions;

use std::mem::size_of;
use test::{black_box, Bencher};
use rand::{XorShiftRng, StdRng, IsaacRng, Isaac64Rng, JitterRng, Rng};
use rand::{OsRng, sample, weak_rng};

#[bench]
fn rand_jitter(b: &mut Bencher) {
    let mut rng = JitterRng::new().unwrap();
    b.iter(|| {
        black_box(rng.next_u64());
    });
    b.bytes = size_of::<u64>() as u64;
}

#[bench]
fn rand_xorshift(b: &mut Bencher) {
    let mut rng: XorShiftRng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_isaac(b: &mut Bencher) {
    let mut rng: IsaacRng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_isaac64(b: &mut Bencher) {
    let mut rng: Isaac64Rng = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_std(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = size_of::<usize>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_f32(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.next_f32());
        }
    });
    b.bytes = size_of::<f32>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_f64(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.next_f64());
        }
    });
    b.bytes = size_of::<f64>() as u64 * RAND_BENCH_N;
}
