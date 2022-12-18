use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j18;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j18_p1", |b| b.iter(|| j18::_p1(black_box(include_str!("j18/j18.txt")))));
    c.bench_function("bench_j18_p2", |b| b.iter(|| j18::_p2(black_box(include_str!("j18/j18.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);