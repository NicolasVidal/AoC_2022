use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j15;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j15_p1", |b| b.iter(|| j15::_p1(black_box(include_str!("j15/j15.txt")))));
    c.bench_function("bench_j15_p2", |b| b.iter(|| j15::_p2(black_box(include_str!("j15/j15.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);