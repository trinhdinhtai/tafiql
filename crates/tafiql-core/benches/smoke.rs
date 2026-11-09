//! Smoke benchmark for the Performance Budget harness (ADR 0004).
//!
//! Benchmarks the operation the Windowed Fetch engine will repeat on every
//! scroll frame: serving a keyset window from the simulated 10M-row source.
//! This is a smoke test that the harness works end to end; real grid and
//! fetch benchmarks land with the engine itself.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use tafiql_core::{RowKey, SimulatedSource};

fn bench_keyset_windows(c: &mut Criterion) {
    let source = SimulatedSource::ten_million_rows();

    let mut group = c.benchmark_group("simulated_source");
    group.throughput(criterion::Throughput::Elements(
        source.window(None, 500).len() as u64,
    ));

    group.bench_function("first_window_500", |b| {
        b.iter(|| source.window(black_box(None), black_box(500)))
    });

    group.bench_function("deep_window_500_near_10m", |b| {
        b.iter(|| source.window(black_box(Some(RowKey(9_999_500))), black_box(500)))
    });

    group.finish();
}

criterion_group!(benches, bench_keyset_windows);
criterion_main!(benches);
