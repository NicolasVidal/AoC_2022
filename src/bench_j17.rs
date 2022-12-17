use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j17;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j17_p1", |b| b.iter(|| j17::_p1(black_box(include_str!("j17/j17.txt")))));
    c.bench_function("bench_j17_p2", |b| b.iter(|| j17::_p2(black_box(include_str!("j17/j17.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);