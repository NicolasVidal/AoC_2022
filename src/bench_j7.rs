use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j7;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j7_p1", |b| b.iter(|| j7::_p1(black_box(include_str!("j7/j7.txt")))));
    c.bench_function("bench_j7_p2", |b| b.iter(|| j7::_p2(black_box(include_str!("j7/j7.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);