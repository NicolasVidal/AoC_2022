use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j14;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j14_p1", |b| b.iter(|| j14::_p1(black_box(include_str!("j14/j14.txt")))));
    c.bench_function("bench_j14_p2", |b| b.iter(|| j14::_p2(black_box(include_str!("j14/j14.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);