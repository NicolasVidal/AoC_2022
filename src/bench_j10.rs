use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j10;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j10_p1", |b| b.iter(|| j10::_p1(black_box(include_str!("j10/j10.txt")))));
    c.bench_function("bench_j10_p2", |b| b.iter(|| j10::_p2(black_box(include_str!("j10/j10.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);