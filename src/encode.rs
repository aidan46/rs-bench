use anyhow::Result;
use reed_solomon_simd::ReedSolomonEncoder;

use crate::config::Config;
use crate::shards::StripeShards;

pub fn encode_stripe(data: &[u8], cfg: &Config) -> Result<StripeShards> {
    let mut original = Vec::with_capacity(cfg.original_shards);
    for chunk in data.chunks(cfg.shard_size).take(cfg.original_shards) {
        let mut shard = vec![0u8; cfg.shard_size];
        shard[..chunk.len()].copy_from_slice(chunk);
        original.push(shard.into_boxed_slice());
    }
    while original.len() < cfg.original_shards {
        original.push(vec![0u8; cfg.shard_size].into_boxed_slice());
    }

    let mut encoder =
        ReedSolomonEncoder::new(cfg.original_shards, cfg.recovery_shards, cfg.shard_size)?;
    for shard in &original {
        encoder.add_original_shard(shard)?;
    }

    let result = encoder.encode()?;
    let recovery = result
        .recovery_iter()
        .map(|s| {
            let mut shard = vec![0u8; cfg.shard_size];
            shard.copy_from_slice(s);
            shard.into_boxed_slice()
        })
        .collect();

    Ok(StripeShards { original, recovery })
}
