use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j4;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j4_p1", |b| b.iter(|| j4::_p1(black_box(include_str!("j4/j4.txt")))));
    c.bench_function("bench_j4_p2", |b| b.iter(|| j4::_p2(black_box(include_str!("j4/j4.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);