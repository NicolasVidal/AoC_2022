use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j6;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j6_p1", |b| b.iter(|| j6::_p1(black_box(include_str!("j6/j6.txt")))));
    c.bench_function("bench_j6_p2", |b| b.iter(|| j6::_p2(black_box(include_str!("j6/j6.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);