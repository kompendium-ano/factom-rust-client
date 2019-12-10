#[macro_use]
extern crate criterion;
extern crate factom;

use criterion::Criterion;
use criterion::black_box;
use factom::*;

fn cloning_benchmark(c: &mut Criterion) {
  let api = Factom::new();
  c.bench_function("Factom struct clone", 
    |b| b.iter(|| black_box(api.clone())));
}

// Benchmarks need to be added to the criterion group
criterion_group!(benches, cloning_benchmark);

// Criterion main runs the group benches
criterion_main!(benches);