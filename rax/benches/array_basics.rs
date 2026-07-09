use criterion::{criterion_group, criterion_main, Criterion};

fn bench_vec_map(c: &mut Criterion) {
    c.bench_function("vec_map_1e6", |b| {
        b.iter(|| {
            let v: Vec<f32> = (0..1_000_000).map(|i| i as f32).collect();
            let _w: Vec<f32> = v.iter().map(|x| x * 1.001 + 3.14).collect();
        })
    });
}

criterion_group!(benches, bench_vec_map);
criterion_main!(benches);