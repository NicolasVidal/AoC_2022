use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j19;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j19_p1", |b| b.iter(|| j19::_p1(black_box(include_str!("j19/j19.txt")))));
    c.bench_function("bench_j19_p2", |b| b.iter(|| j19::_p2(black_box(include_str!("j19/j19.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);