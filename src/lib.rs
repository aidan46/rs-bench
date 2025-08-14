mod config;
mod decode;
mod encode;
mod shards;

pub use config::{Config, generate_configs};
pub use decode::decode_stripe;
pub use encode::encode_stripe;
pub use shards::StripeShards;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;
pub const GB: usize = 1024 * MB;

pub fn format_size(bytes: usize) -> String {
    match bytes {
        b if b >= GB => format!("{:.1} GB", b as f64 / (1 << 30) as f64),
        b if b >= MB => format!("{:.1} MB", b as f64 / (1 << 20) as f64),
        b if b >= KB => format!("{:.1} KB", b as f64 / (1 << 10) as f64),
        _ => format!("{bytes} B"),
    }
}
