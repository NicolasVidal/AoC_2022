use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j11;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j11_p1", |b| b.iter(|| j11::_p1(black_box(include_str!("j11/j11.txt")))));
    c.bench_function("bench_j11_p2", |b| b.iter(|| j11::_p2(black_box(include_str!("j11/j11.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);