use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j5;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j5_p1", |b| b.iter(|| j5::_p1(black_box(include_str!("j5/j5.txt")))));
    c.bench_function("bench_j5_p2", |b| b.iter(|| j5::_p2(black_box(include_str!("j5/j5.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);