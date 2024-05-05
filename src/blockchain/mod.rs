use super::block;
use chrono::Utc;

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

    pub fn add_block(&mut self, data: String) -> &mut Self {
        let last_block = self.chain.last().unwrap();
        let previous_hash = last_block.hash.clone();
        let new_index = last_block.index + 1;
        let block = block::Block::new(new_index, Utc::now(), data, previous_hash);
        self.chain.push(block);
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
}
