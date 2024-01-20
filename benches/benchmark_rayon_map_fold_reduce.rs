use std::fs::read_to_string;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use obrc::solutions::rayon_map_fold_reduce::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input: String = read_to_string("measurements_1000000000.txt").unwrap();

    c.bench_function("rayon_map_fold_reduce", |b| {
        b.iter(|| solve(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
