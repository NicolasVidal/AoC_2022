use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j24;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j24_p1", |b| b.iter(|| j24::_p1(black_box(include_str!("j24/j24.txt")))));
    c.bench_function("bench_j24_p2", |b| b.iter(|| j24::_p2(black_box(include_str!("j24/j24.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);