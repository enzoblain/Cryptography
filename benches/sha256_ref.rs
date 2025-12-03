use criterion::{Criterion, criterion_group, criterion_main};
use sha2::{Digest, Sha256};
use std::hint::black_box;
use std::time::Instant;

pub fn bench_sha2_ref(c: &mut Criterion) {
    c.bench_function("sha256_ref", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();

            for _ in 0..iters {
                let mut hasher = Sha256::new();
                hasher.update(black_box(&black_box(&[0u8; 64])));
                let _ = hasher.finalize();
            }

            start.elapsed()
        });
    });
}

criterion_group!(benches, bench_sha2_ref);
criterion_main!(benches);
