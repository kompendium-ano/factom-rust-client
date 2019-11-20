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

criterion_group!(benches, cloning_benchmark);
criterion_main!(benches);