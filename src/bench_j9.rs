use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j9;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j9_p1", |b| b.iter(|| j9::_p1(black_box(include_str!("j9/j9.txt")))));
    c.bench_function("bench_j9_p2", |b| b.iter(|| j9::_p2(black_box(include_str!("j9/j9.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);