use anyhow::Result;
use reed_solomon_simd::ReedSolomonDecoder;

use crate::config::Config;
use crate::shards::StripeShards;

pub fn decode_stripe(shards: &StripeShards, original_len: usize, cfg: &Config) -> Result<Vec<u8>> {
    let mut decoder =
        ReedSolomonDecoder::new(cfg.original_shards, cfg.recovery_shards, cfg.shard_size)?;
    let mut added = 0;

    for (i, shard) in shards.original.iter().enumerate() {
        decoder.add_original_shard(i, shard)?;
        added += 1;
        if added >= cfg.original_shards {
            break;
        }
    }

    if added < cfg.original_shards {
        for (i, shard) in shards.recovery.iter().enumerate() {
            decoder.add_recovery_shard(i, shard)?;
            added += 1;
            if added >= cfg.original_shards {
                break;
            }
        }
    }

    let result = decoder.decode()?;
    let restored = result
        .restored_original_iter()
        .collect::<std::collections::HashMap<_, _>>();

    let full_original: Vec<&[u8]> = shards
        .original
        .iter()
        .enumerate()
        .map(|(i, shard)| {
            restored
                .get(&i)
                .map(|s| s.as_ref())
                .unwrap_or_else(|| shard.as_ref())
        })
        .collect();

    let mut out = Vec::with_capacity(original_len);
    for shard in full_original {
        out.extend_from_slice(shard);
    }

    out.truncate(original_len);
    Ok(out)
}
