use super::block;
use chrono::Utc;
use std::{fs::{OpenOptions}, io::Write};

pub struct BlockChain {
    chain: Vec<block::Block>,
    file_path: String,
}

impl BlockChain {
    pub fn new(path: String) -> Self {
        BlockChain {
            chain: vec![block::Block::new(
                0,
                Utc::now(),
                "Genesis Block".to_string(),
                "0".to_string(),
            )],
            file_path: path,
        }
    }

    pub fn add_block(&mut self, data: String) -> &mut Self {
        let last_block = self.chain.last().unwrap();
        let previous_hash = last_block.hash.clone();
        let new_index = last_block.index + 1;
        let block = block::Block::new(new_index, Utc::now(), data, previous_hash);
        self.chain.push(block);
        self.save_chain_to_disk();
        self
    }

    pub fn validate_chain(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate().skip(1) {
            let block_hash = &block.hash;
            let recalculated_hash = block::Block::calculate_hash(
                block.index,
                &block.timestamp,
                &block.data,
                &block.previous_hash,
            );

            // Check if the current block's hash matches the recalculated hash
            // and if the current block's previous hash matches the previous block's hash
            // to ensure that the chain is not tampered
            if block_hash != &recalculated_hash
                || block.previous_hash != self.chain.get(i - 1).unwrap().hash
            {
                return false;
            }
        }
        return true;
    }

    fn save_chain_to_disk(&self) {
      let mut file = match OpenOptions::new()
          .write(true)
          .create(true)
          .open(&self.file_path) {
              Ok(file) => file,
              Err(e) => {
                  eprintln!("Failed to open or create the file: {}", e);
                  return;
              }
          };
  
      let json_string = match serde_json::to_string(&self.chain) {
          Ok(json) => json,
          Err(e) => {
              eprintln!("Failed to serialize chain: {}", e);
              return;
          }
      };
  
      if let Err(e) = writeln!(file, "{}", json_string) {
          eprintln!("Couldn't write to file: {}", e);
      }
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blockchain() {
        let blockchain = BlockChain::new("./test.json".to_string());
        assert_eq!(blockchain.chain.len(), 1); // Check if genesis block is created
        assert_eq!(blockchain.chain[0].data, "Genesis Block");
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = BlockChain::new("./test.json".to_string());
        blockchain.add_block("Test Data".to_string());
        assert_eq!(blockchain.chain.len(), 2); // Should have genesis and one added block
        assert_eq!(blockchain.chain[1].data, "Test Data");
    }

    #[test]
    fn test_validate_chain() {
        let mut blockchain = BlockChain::new("./test.json".to_string());
        blockchain.add_block("Block 1".to_string());
        blockchain.add_block("Block 2".to_string());
        assert!(blockchain.validate_chain()); // Check if the blockchain is valid after additions

        // Tamper with the blockchain
        blockchain.chain[1].data = "Tampered Data".to_string();
        assert!(!blockchain.validate_chain()); // Validation should fail after tampering
    }
}
