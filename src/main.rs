use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use itertools::Itertools;

use crate::benchmark::{BenchmarkResult, benchmark_config};
use crate::config::generate_configs;

mod benchmark;
mod config;
mod decode;
mod encode;
mod shards;

/// Format duration as ms if <1s, otherwise as seconds.
fn format_duration(dur: Duration) -> String {
    let secs = dur.as_secs_f64();
    if secs < 1.0 {
        format!("{:.2} ms", secs * 1000.0)
    } else {
        format!("{secs:.2} s")
    }
}

fn format_size(bytes: usize) -> String {
    match bytes {
        b if b >= 1 << 30 => format!("{:.1} GB", b as f64 / (1 << 30) as f64),
        b if b >= 1 << 20 => format!("{:.1} MB", b as f64 / (1 << 20) as f64),
        b if b >= 1 << 10 => format!("{:.1} KB", b as f64 / (1 << 10) as f64),
        _ => format!("{bytes} B"),
    }
}

fn write_markdown_table(results: &[BenchmarkResult]) -> Result<()> {
    let mut file = File::create("README.md")?;

    let mut best_by_file_size: HashMap<usize, &BenchmarkResult> = HashMap::new();

    for r in results {
        best_by_file_size
            .entry(r.file_size)
            .and_modify(|best| {
                if r.total_time < best.total_time {
                    *best = r;
                }
            })
            .or_insert(r);
    }

    writeln!(file, "# Summary: Best Configs Per File Size\n")?;

    for (size, result) in best_by_file_size.iter().sorted_by_key(|(k, _)| *k) {
        writeln!(
            file,
            "- **{}** → `{}` shard size, `{}` shards ({} original + {} recovery) → Total: {}, Encode: {}, Decode: {}",
            format_size(*size),
            format_size(result.shard_size),
            result.original_shards + result.recovery_shards,
            result.original_shards,
            result.recovery_shards,
            format_duration(result.total_time),
            format_duration(result.encode_time),
            format_duration(result.decode_time),
        )?;
    }

    writeln!(
        file,
        "| {:<10} | {:<10} | {:<14} | {:>11} | {:>11} | {:>10} | {:>9} | {:>9} |",
        "File Size",
        "Shard Size",
        "Shards (O+R)",
        "Encode",
        "Decode",
        "Total",
        "Enc MB/s",
        "Dec MB/s"
    )?;
    writeln!(
        file,
        "|------------|------------|----------------|-------------|-------------|------------|-----------|-----------|"
    )?;

    for r in results {
        let mb = r.file_size as f64 / 1_048_576.0;
        let enc_tput = mb / r.encode_time.as_secs_f64();
        let dec_tput = mb / r.decode_time.as_secs_f64();

        writeln!(
            file,
            "| {:<10} | {:<10} | {:<14} | {:>11} | {:>11} | {:>10} | {:>9.2} | {:>9.2} |",
            format_size(r.file_size),
            format_size(r.shard_size),
            format!("{}+{}", r.original_shards, r.recovery_shards),
            format_duration(r.encode_time),
            format_duration(r.decode_time),
            format_duration(r.total_time),
            enc_tput,
            dec_tput,
        )?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let configs = generate_configs();
    let mut results = vec![];

    println!("Starting benchmarking...");
    for config in &configs {
        let result = benchmark_config(config)?;
        results.push(result);
    }

    println!("Sorting results...");
    // Sort results by total time ascending
    results.sort_by_key(|r| r.total_time);

    println!("Writing results...");
    // Write results to markdown file
    write_markdown_table(&results)?;

    Ok(())
}
