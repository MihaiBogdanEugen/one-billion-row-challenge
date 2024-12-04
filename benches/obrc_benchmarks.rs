use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Bencher;
use criterion::Criterion;
use obrc::solutions::solution_basic::SolutioBasic;
use obrc::solutions::solution_naive::SolutioNaive;
use obrc::solutions::solution_rayon_fxhash::SolutionRayonFxHash;
use obrc::solutions::solver::Solver;

fn benchmark_solution_basic(c: &mut Criterion) {
    c.bench_function("SolutioBasic", |b: &mut Bencher<'_>| {
        b.iter(|| SolutioBasic::solve_obrc(black_box("resources/measurements_1000000.txt")))
    });
}

fn benchmark_solution_naive(c: &mut Criterion) {
    c.bench_function("SolutioNaive", |b: &mut Bencher<'_>| {
        b.iter(|| SolutioNaive::solve_obrc(black_box("resources/measurements_1000000.txt")))
    });
}

fn benchmark_solution_rayon_fxhash(c: &mut Criterion) {
    c.bench_function("SolutionRayonFxHash", |b: &mut Bencher<'_>| {
        b.iter(|| SolutionRayonFxHash::solve_obrc(black_box("resources/measurements_1000000.txt")))
    });
}

criterion_group! {
    name = benches_basic;
    config = Criterion::default().sample_size(20);
    targets = benchmark_solution_basic
}

criterion_group! {
    name = benches_naive;
    config = Criterion::default().sample_size(20);
    targets = benchmark_solution_naive
}

criterion_group! {
    name = benches_rayon_fxhash;
    config = Criterion::default().sample_size(20);
    targets = benchmark_solution_rayon_fxhash
}

criterion_main!(benches_basic, benches_naive, benches_rayon_fxhash);
