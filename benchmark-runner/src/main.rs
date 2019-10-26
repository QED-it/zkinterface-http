#![feature(test)]
extern crate test;

use test::test::Bencher;

mod circuit;
mod runner;

#[bench]
fn bench_1_bulletproofs(b: &mut Bencher) {
    runner::run(b, "cd ../bulletproofs-runner && cargo run", 1).unwrap();
}

#[bench]
fn bench_10_bulletproofs(b: &mut Bencher) {
    runner::run(b, "cd ../bulletproofs-runner && cargo run", 10).unwrap();
}

#[bench]
fn bench_100_bulletproofs(b: &mut Bencher) {
    runner::run(b, "cd ../bulletproofs-runner && cargo run", 100).unwrap();
}
