use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j25;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j25_p1", |b| b.iter(|| j25::_p1(black_box(include_str!("j25/j25.txt")))));
    c.bench_function("bench_j25_p2", |b| b.iter(|| j25::_p2(black_box(include_str!("j25/j25.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);