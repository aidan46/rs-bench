use criterion::{
    BenchmarkId, Criterion, SamplingMode, Throughput, criterion_group, criterion_main,
};
use reed_solomon::{decode_stripe, encode_stripe, format_size, generate_configs};
use std::time::Duration;

fn encode_decode_group(c: &mut Criterion) {
    // One report page for everything
    let mut group = c.benchmark_group("reed-solomon");

    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(30))
        .sample_size(10)
        .sampling_mode(SamplingMode::Flat);

    for cfg in generate_configs() {
        // Handy label shown in the report
        let label = format!(
            "{} / {}+{} / {}",
            format_size(cfg.file_size),
            cfg.original_shards,
            cfg.recovery_shards,
            format_size(cfg.shard_size)
        );

        // Precompute data and encoded shards *once per config*
        let data = vec![0u8; cfg.file_size];
        let stripe_size = cfg.original_shards * cfg.shard_size;

        // Throughput per benchmark: full file MB/sec
        group.throughput(Throughput::Bytes(cfg.file_size as u64));

        // ---------- Encode ----------
        group.bench_with_input(BenchmarkId::new("encode", &label), &cfg, |b, cfg| {
            b.iter(|| {
                let mut local_encoded = Vec::new();
                for chunk in data.chunks(stripe_size) {
                    let shards = encode_stripe(chunk, cfg).unwrap();
                    local_encoded.push((shards, chunk.len()));
                }
                local_encoded
            })
        });

        // Pre-encode once for the decode bench
        let mut encoded = Vec::new();
        for chunk in data.chunks(stripe_size) {
            let shards = encode_stripe(chunk, &cfg).unwrap();
            encoded.push((shards, chunk.len()));
        }

        // ---------- Decode ----------
        group.bench_with_input(BenchmarkId::new("decode", &label), &cfg, |b, cfg| {
            b.iter(|| {
                let mut restored = Vec::with_capacity(cfg.file_size);
                for (shards, len) in &encoded {
                    let out = decode_stripe(shards, *len, cfg).unwrap();
                    restored.extend(out);
                }
                restored
            })
        });
    }

    group.finish();
}

fn long_run_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(10))
        .sample_size(20)
}

criterion_group! {
    name = benches;
    config = long_run_criterion();
    targets = encode_decode_group
}
criterion_main!(benches);
