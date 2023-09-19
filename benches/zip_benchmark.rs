use criterion::{criterion_group, criterion_main, Criterion};

fn my_benchmark_function(c: &mut Criterion) {
    c.bench_function("my_benchmark", |b| {
        b.iter(|| {
            // Code to be benchmarked
            println!("hello");
            
        });
    });
}

criterion_group!(benches, my_benchmark_function);
criterion_main!(benches);
