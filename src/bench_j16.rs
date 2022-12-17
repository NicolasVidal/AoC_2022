use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j16;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j16_p1", |b| b.iter(|| j16::_p1(black_box(include_str!("j16/j16.txt")))));
    c.bench_function("bench_j16_p2", |b| b.iter(|| j16::_p2(black_box(include_str!("j16/j16.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);