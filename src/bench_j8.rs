use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j8;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j8_p1", |b| b.iter(|| j8::_p1(black_box(include_str!("j8/j8.txt")))));
    c.bench_function("bench_j8_p2", |b| b.iter(|| j8::_p2(black_box(include_str!("j8/j8.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);