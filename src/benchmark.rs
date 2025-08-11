use std::time::{Duration, Instant};

use anyhow::Result;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serde::Serialize;

use crate::config::Config;
use crate::decode::decode_stripe;
use crate::encode::encode_stripe;

#[derive(Serialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub file_size: usize,
    pub shard_size: usize,
    pub original_shards: usize,
    pub recovery_shards: usize,
    pub encode_time: Duration,
    pub decode_time: Duration,
    pub total_time: Duration,
}

pub fn benchmark_config(config: &Config) -> Result<BenchmarkResult> {
    let data = generate_random_bytes(config.file_size);
    let stripe_size = config.original_shards * config.shard_size;
    let mut encoded = vec![];
    let mut restored = Vec::with_capacity(config.file_size);

    let total_start = Instant::now();

    let start_encode = Instant::now();
    for chunk in data.chunks(stripe_size) {
        let shards = encode_stripe(chunk, config)?;
        encoded.push((shards, chunk.len()));
    }
    let encode_time = start_encode.elapsed();

    let start_decode = Instant::now();
    for (shards, chunk_len) in &encoded {
        let decoded = decode_stripe(shards, *chunk_len, config)?;
        restored.extend(decoded);
    }
    let decode_time = start_decode.elapsed();

    let total_time = total_start.elapsed();
    assert_eq!(&restored[..], &data[..]);

    Ok(BenchmarkResult {
        name: config.name.clone(),
        file_size: config.file_size,
        shard_size: config.shard_size,
        original_shards: config.original_shards,
        recovery_shards: config.recovery_shards,
        encode_time,
        decode_time,
        total_time,
    })
}

fn generate_random_bytes(len: usize) -> Vec<u8> {
    let mut data = vec![0u8; len];
    let mut rng = StdRng::seed_from_u64(42); // deterministic for consistent benchmarks
    rng.fill_bytes(&mut data);
    data
}
