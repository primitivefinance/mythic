use criterion::*;

fn basic_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_benchmark");
    group.bench_function("basic_benchmark", |b| b.iter(|| 1 + 2));
    group.finish();
}

criterion_group!(benches, basic_benchmark);
criterion_main!(benches);
