use criterion::{black_box, criterion_group,SamplingMode, criterion_main, Criterion};
use dstrombe_simd::mandelbrot::{render_mandelbrot, ComplexNum};


fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark_simd");
    group.sample_size(100);
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("Mandelbrot compute, simd: 512x512", |b| b.iter(|| render_mandelbrot(true, ComplexNum {
        re : -1.0,
        im : -1.0,
    }, 2.0, 512, 200)));
    group.finish();
    let mut group = c.benchmark_group("benchmark_sisd");
    group.sample_size(100);
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("Mandelbrot compute, sisd: 512x512", |b| b.iter(|| render_mandelbrot(false, ComplexNum {
        re : -1.0,
        im : -1.0,
    }, 2.0, 512, 200)));
    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
