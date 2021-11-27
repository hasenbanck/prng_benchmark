use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use prng_benchmark::*;

pub fn scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar");

    let lehmer64 = Lehmer64::default();
    let mwc256xxa64 = Mwc256XXA64::default();
    let pcg64 = Pcg64::default();
    let pcg64fast = Pcg64Fast::default();
    let tylo64 = Tylo64::default();
    let romu_jr = RomuJr::default();
    let romu_trio = RomuTrio::default();
    let splitmix64 = Splitmix64::default();
    let wyrand = Wyrand::default();
    let xoshiro256pp = Xoshiro256plusplus::default();

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("lehmer64", "u64"), |b| {
        b.iter(|| {
            let x = lehmer64.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("mwc256xxa64", "u64"), |b| {
        b.iter(|| {
            let x = mwc256xxa64.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("pcg64fast", "u64"), |b| {
        b.iter(|| {
            let x = pcg64fast.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("pcg64", "u64"), |b| {
        b.iter(|| {
            let x = pcg64.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("splitmix64", "u64"), |b| {
        b.iter(|| {
            let x = splitmix64.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("romu_jr", "u64"), |b| {
        b.iter(|| {
            let x = romu_jr.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("romu_trio", "u64"), |b| {
        b.iter(|| {
            let x = romu_trio.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("tylo64", "u64"), |b| {
        b.iter(|| {
            let x = tylo64.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("wyrand", "u64"), |b| {
        b.iter(|| {
            let x = wyrand.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("xoshiro256++", "u64"), |b| {
        b.iter(|| {
            let x = xoshiro256pp.u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    group.finish()
}

pub fn bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes");
    let count = 1024 * 1024;
    group.throughput(Throughput::Bytes(count));

    let lehmer64 = Lehmer64::default();
    let mwc256xxa64 = Mwc256XXA64::default();
    let pcg64fast = Pcg64Fast::default();
    let pcg64 = Pcg64::default();
    let tylo = Tylo64::default();
    let romu_jr = RomuJr::default();
    let romu_trio = RomuTrio::default();
    let splitmix = Splitmix64::default();
    let wyrand = Wyrand::default();
    let xoshiro256pp = Xoshiro256plusplus::default();

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("lehmer64", "1MiB"), |b| {
        b.iter(|| {
            lehmer64.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("mwc256xxa64", "1MiB"), |b| {
        b.iter(|| {
            mwc256xxa64.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("pcg64fast", "1MiB"), |b| {
        b.iter(|| {
            pcg64fast.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("pcg64", "1MiB"), |b| {
        b.iter(|| {
            pcg64.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("romu_jr", "1MiB"), |b| {
        b.iter(|| {
            romu_jr.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("romu_trio", "1MiB"), |b| {
        b.iter(|| {
            romu_trio.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("splitmix64", "1MiB"), |b| {
        b.iter(|| {
            splitmix.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("tylo64", "u64"), |b| {
        b.iter(|| {
            tylo.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("wyrand", "1MiB"), |b| {
        b.iter(|| {
            wyrand.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("xoshiro256++", "1MiB"), |b| {
        b.iter(|| {
            xoshiro256pp.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    group.finish();
}

pub fn thread_local(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_local");

    lehmer64_seed();
    mwc256xxa64_seed();
    pcg64_seed();
    pcg64fast_seed();
    tylo64_seed();
    romu_jr_seed();
    romu_trio_seed();
    splitmix64_seed();
    wyrand_seed();
    xoshiro256plusplus_seed();

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("lehmer64", "u64"), |b| {
        b.iter(|| {
            let x = lehmer64_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("mwc256xxa64", "u64"), |b| {
        b.iter(|| {
            let x = mwc256xxa64_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("pcg64fast", "u64"), |b| {
        b.iter(|| {
            let x = pcg64fast_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("pcg64", "u64"), |b| {
        b.iter(|| {
            let x = pcg64_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("splitmix64", "u64"), |b| {
        b.iter(|| {
            let x = splitmix64_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("romu_jr", "u64"), |b| {
        b.iter(|| {
            let x = romu_jr_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("romu_trio", "u64"), |b| {
        b.iter(|| {
            let x = romu_trio_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("tylo64", "u64"), |b| {
        b.iter(|| {
            let x = tylo64_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("wyrand", "u64"), |b| {
        b.iter(|| {
            let x = wyrand_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    let mut state: u64 = 0;
    group.bench_function(BenchmarkId::new("xoshiro256++", "u64"), |b| {
        b.iter(|| {
            let x = xoshiro256plusplus_u64();
            state = state.wrapping_add(x);
        })
    });
    black_box(state);

    group.finish()
}

criterion_group!(benches, scalar, bytes, thread_local);
criterion_main!(benches);
