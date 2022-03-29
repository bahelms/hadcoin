use serde::Serialize;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

const HASH_TARGET: &str = "000";

#[derive(Clone, Serialize)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Self { chain: Vec::new() };
        let mut block = chain.create_block("0".to_string());
        block.calculate_proof_of_work();
        chain.chain.push(block);
        chain
    }

    fn create_block(&self, previous_hash: String) -> Block {
        Block::new(self.chain.len() + 1, previous_hash)
    }

    fn previous_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn is_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i + 1 < self.chain.len() && self.chain[i + 1].previous_hash != hash(&block) {
                return false;
            }
            if !meets_target(&block) {
                return false;
            }
        }
        true
    }

    pub fn mine_block(&mut self) -> Option<&Block> {
        match self.previous_block() {
            Some(previous_block) => {
                let mut block = self.create_block(hash(previous_block));
                block.calculate_proof_of_work();
                self.chain.push(block);
                self.chain.last()
            }
            None => panic!("Can't mine a new block without a genesis block"),
        }
    }
}

impl fmt::Debug for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[").unwrap();
        for block in &self.chain {
            writeln!(f, "\t{:?}", block).unwrap();
        }
        writeln!(f, "]")
    }
}

fn meets_target(block: &Block) -> bool {
    &hash(block)[..HASH_TARGET.len()] == HASH_TARGET
}

pub fn hash(block: &Block) -> String {
    let json = match serde_json::to_string(block) {
        Ok(json) => json,
        _ => panic!("{:?} failed to serialize", block),
    };
    sha256::digest(json)
}

#[derive(Debug, Serialize, Clone)]
pub struct Block {
    index: usize,
    previous_hash: String,
    nonce: i32,
    timestamp: u64, // seconds since UNIX_EPOCH
}

impl Block {
    fn new(index: usize, previous_hash: String) -> Self {
        Self {
            index,
            previous_hash,
            nonce: 1,
            timestamp: current_time(),
        }
    }

    fn calculate_proof_of_work(&mut self) {
        while !meets_target(self) {
            self.nonce += 1;
        }
    }
}

fn current_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        _ => panic!("SystemTime not found"),
    }
}

#[cfg(test)]
mod tests;
