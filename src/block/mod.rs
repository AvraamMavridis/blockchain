use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_create_new_block() {
        let index = 0;
        let timestamp = Utc.with_ymd_and_hms(2023, 9, 30, 12, 0, 0).unwrap();// Fixed timestamp for reproducibility
        let data = "Example data";
        let previous_hash = "abcdef1234567890";

        let block = Block::new(index, timestamp, data.to_string(), previous_hash.to_string());

        // Check that all fields are set correctly
        assert_eq!(block.index, index);
        assert_eq!(block.timestamp, timestamp);
        assert_eq!(block.data, data);
        assert_eq!(block.previous_hash, previous_hash);

        // Check hash calculation
        let expected_hash = Block::calculate_hash(index, &timestamp, data, previous_hash);
        assert_eq!(block.hash, expected_hash);
    }
}
