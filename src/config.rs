const KB: usize = 1024;
const MB: usize = 1024 * KB;
const GB: usize = 1024 * MB;

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub file_size: usize,
    pub shard_size: usize,
    pub original_shards: usize,
    pub recovery_shards: usize,
}

pub fn generate_configs() -> Vec<Config> {
    let file_sizes = [
        (128 * MB, "128MB"),
        (256 * MB, "256MB"),
        (512 * MB, "512MB"),
        (GB, "1GB"),
        (2 * GB, "2GB"),
        (4 * GB, "4GB"),
    ];

    let shard_sizes = [
        (128 * KB, "128KB"),
        (256 * KB, "256KB"),
        (512 * KB, "512KB"),
        (MB, "1MB"),
        (2 * MB, "2MB"),
        (4 * MB, "4MB"),
    ];

    // (original, recovery) shard count pairs
    let shard_counts = [
        (4, 2),  // light config
        (6, 3),  // 33% redundancy
        (10, 4), // 40% redundancy
        (16, 6), // heavier config
        (20, 8), // very heavy config
    ];

    let mut configs = vec![];

    for &(file_size, fs_label) in &file_sizes {
        for &(shard_size, ss_label) in &shard_sizes {
            for &(orig, recov) in &shard_counts {
                let name = format!("{fs_label} / {orig}+{recov} / {ss_label}");

                configs.push(Config {
                    name,
                    file_size,
                    shard_size,
                    original_shards: orig,
                    recovery_shards: recov,
                });
            }
        }
    }

    configs
}
