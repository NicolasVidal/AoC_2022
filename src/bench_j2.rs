use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j2;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j2_p1", |b| b.iter(|| j2::_p1(black_box(include_str!("j2/j2.txt")))));
    c.bench_function("bench_j2_p2", |b| b.iter(|| j2::_p2(black_box(include_str!("j2/j2.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);