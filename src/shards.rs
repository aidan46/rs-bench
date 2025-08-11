/// Represents one stripe's worth of encoded shards.
pub struct StripeShards {
    pub original: Vec<Box<[u8]>>,
    pub recovery: Vec<Box<[u8]>>,
}
