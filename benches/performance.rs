//! Performance benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn terminal_render_benchmark(c: &mut Criterion) {
    c.bench_function("terminal_render", |b| {
        b.iter(|| {
            // Benchmark terminal rendering
            black_box(());
        });
    });
}

fn theme_parse_benchmark(c: &mut Criterion) {
    c.bench_function("theme_parse", |b| {
        b.iter(|| {
            // Benchmark theme parsing
            black_box(());
        });
    });
}

criterion_group!(benches,terminal_render_benchmark,theme_parse_benchmark);
criterion_main!(benches);
