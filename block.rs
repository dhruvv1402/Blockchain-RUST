use crate::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let nonce = 0;
        let hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash, nonce);
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }
    pub fn calculate_hash(index: u64, timestamp: i64, transactions: &Vec<Transaction>, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, index.to_string());
        Digest::update(&mut hasher, timestamp.to_string());
        Digest::update(&mut hasher, format!("{:?}", transactions));
        Digest::update(&mut hasher, previous_hash);
        Digest::update(&mut hasher, nonce.to_string());
        let result = hasher.finalize();
        format!("{:?}", result.as_slice())
    }
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = Block::calculate_hash(
                self.index,
                self.timestamp,
                &self.transactions,
                &self.previous_hash,
                self.nonce,
            );
        }
        println!("Block mined: {}", self.hash);
    }
}
