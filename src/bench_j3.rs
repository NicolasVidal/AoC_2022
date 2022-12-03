use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j3;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j3_p1", |b| b.iter(|| j3::_p1(black_box(include_str!("j3/j3.txt")))));
    c.bench_function("bench_j3_p2", |b| b.iter(|| j3::_p2(black_box(include_str!("j3/j3.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);