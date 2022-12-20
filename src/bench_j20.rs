use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j20;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j20_p1", |b| b.iter(|| j20::_p1(black_box(include_str!("j20/j20.txt")))));
    c.bench_function("bench_j20_p2", |b| b.iter(|| j20::_p2(black_box(include_str!("j20/j20.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);