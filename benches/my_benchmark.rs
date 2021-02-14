use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use benchmarks::{fibonacci_slow, fibonacci_fast};

pub fn benchmark_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [20u64, 21u64].iter() {
       group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| b.iter(|| fibonacci_slow(*i)));
       group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| b.iter(|| fibonacci_fast(*i)));
    }
    group.finish();
}

criterion_group!(benches, benchmark_fibs);
criterion_main!(benches);

