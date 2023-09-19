use criterion::{criterion_group, criterion_main, Criterion};

use zip_test::extract_files;

fn my_benchmark_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-10");

    group.significance_level(0.1).sample_size(10);
    group.bench_function("extract_process_zip_10_1", |b| {
        b.iter(|| {
            // Code to be benchmarked
            extract_files(10,1);
        });
    });
    group.finish();
}

criterion_group!(benches, my_benchmark_function);
criterion_main!(benches);
