use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j23;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_j23_p1", |b| b.iter(|| j23::_p1(black_box(include_str!("j23/j23.txt")))));
    c.bench_function("bench_j23_p2", |b| b.iter(|| j23::_p2(black_box(include_str!("j23/j23.txt")))));
}

criterion_group!(benches, bench);
criterion_main!(benches);