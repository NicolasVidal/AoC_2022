use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j1;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j1_p1", |b| b.iter(|| j1::_p1(black_box(include_str!("j1/j1.txt")))));
    c.bench_function("bench_j1_p2", |b| b.iter(|| j1::_p2(black_box(include_str!("j1/j1.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);