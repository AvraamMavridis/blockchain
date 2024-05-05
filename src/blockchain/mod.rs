use super::block;
use chrono::{Utc};

pub struct BlockChain {
    chain: Vec<block::Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            chain: vec![block::Block::new(
                0,
                Utc::now(),
                "Genesis Block".to_string(),
                "0".to_string(),
            )],
        }
    }

    pub fn add_block(&mut self, data: String) -> & mut Self {
      let last_block = self.chain.last().unwrap();
      let previous_hash = last_block.hash.clone();
      let new_index = last_block.index + 1;
      let block = block::Block::new(new_index, Utc::now(), data, previous_hash);
      self.chain.push(block);
      self
    }
}
