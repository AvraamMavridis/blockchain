use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: DateTime<Utc>, data: String, previous_hash: String) -> Self {
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: &DateTime<Utc>,
        data: &str,
        previous_hash: &str,
    ) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}
