use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j22;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j22_p1", |b| b.iter(|| j22::_p1(black_box(include_str!("j22/j22.txt")))));
    c.bench_function("bench_j22_p2", |b| b.iter(|| j22::_p2(black_box(include_str!("j22/j22.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);