use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};



#[derive(Serialize , Deserialize , Debug)]
pub struct Block{
    pub index : u64,
    pub timestamp : i64,
    pub data : String,
    pub previous_hash : String,
    pub current_hash : String
}

impl Block{
    pub fn calculate_hash(&self) -> String{
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}" , self.index , self.timestamp , self.data , self.previous_hash);
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn new(data:String) -> Self{
        let now = Utc::now();
        let timestamp = now.timestamp();
        let genesis_block = Block{
            index : 1,
            timestamp : timestamp,
            data : data.clone(),
            previous_hash : String::from("Sneha is my girl and i love her so much"),
            current_hash : String::new()
        };
        let hash = genesis_block.calculate_hash();
        Block { index: 1, timestamp: timestamp, data:data , previous_hash: String::from("Sneha is my girl and i love her so much"), current_hash: hash }
    }

    pub fn next(index:u64 , data:String , previous_hash:String) -> Self{
        let now = Utc::now();
        let timestamp = now.timestamp();
        let next_blocks = Block{
            index:index,
            timestamp : timestamp,
            data : data.clone(),
            previous_hash : previous_hash.clone(),
            current_hash : String::new()
        };

        let hash = next_blocks.calculate_hash();

        Block { index: index, timestamp: timestamp, data: data, previous_hash: previous_hash, current_hash: hash }
    }
}