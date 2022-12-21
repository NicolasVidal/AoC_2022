use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j21;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j21_p1", |b| b.iter(|| j21::_p1(black_box(include_str!("j21/j21.txt")))));
    c.bench_function("bench_j21_p2", |b| b.iter(|| j21::_p2(black_box(include_str!("j21/j21.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);