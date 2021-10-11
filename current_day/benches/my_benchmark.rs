use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/common.rs"]
mod common;

#[path = "../src/day/mod.rs"]
mod day;

fn criterion_benchmark(c: &mut Criterion) {
    if let Ok(input) = common::file_to_lines("../src/day/input.txt") {
        c.bench_function("normal p1", |b| b.iter(|| day::p1(&input, false)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);