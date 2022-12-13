use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j13;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j13_p1", |b| b.iter(|| j13::_p1(black_box(include_str!("j13/j13.txt")))));
    c.bench_function("bench_j13_p2", |b| b.iter(|| j13::_p2(black_box(include_str!("j13/j13.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);