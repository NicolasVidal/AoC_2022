use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j12;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j12_p1", |b| b.iter(|| j12::_p1(black_box(include_str!("j12/j12.txt")))));
    c.bench_function("bench_j12_p2", |b| b.iter(|| j12::_p2(black_box(include_str!("j12/j12.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);