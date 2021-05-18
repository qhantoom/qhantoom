#[macro_use]
extern crate criterion;

use qhantoom::front::tokenizer::{
  tokenize_capsule_from_file, tokenize_capsule_from_source,
};

use std::path::Path;

use criterion::{black_box, Criterion};

fn tokenize_capsule_from_file_benchmarks(c: &mut Criterion) {
  let path = Path::new("samples/data/entry/simple.qh");
  run_tokenize_capsule_from_file_bench(c, &path);
}

fn tokenize_capsule_from_source_benchmarks(c: &mut Criterion) {
  let sample = r#"fun sqrt: Fun(unit) -> uint = (x) { x * x }"#;
  run_tokenize_capsule_from_source_bench(c, sample);
}

fn run_tokenize_capsule_from_file_bench(c: &mut Criterion, file: &Path) {
  c.bench_function(&"tokenize_capsule_from_file", move |b| {
    b.iter(|| {
      let tokens = tokenize_capsule_from_file(file).unwrap();
      let _ = black_box(tokens);
    });
  });
}

fn run_tokenize_capsule_from_source_bench(c: &mut Criterion, src: &str) {
  c.bench_function(&"tokenize_capsule_from_source", move |b| {
    b.iter(|| {
      let tokens = tokenize_capsule_from_source(src).unwrap();
      let _ = black_box(tokens);
    });
  });
}

criterion_group!(
  benches,
  tokenize_capsule_from_file_benchmarks,
  tokenize_capsule_from_source_benchmarks,
);

criterion_main!(benches);
