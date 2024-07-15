use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub blocks: Vec<Block>,  // Vec<Block>으로 수정
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

fn main() {
    // 예제 데이터 생성
    let block1 = Block {
        id: 1,
        hash: String::from("hash1"),
        previous_hash: String::from("prev_hash1"),
        timestamp: 123456789,
        data: String::from("Block 1 data"),
        nonce: 0,
    };
    let block2 = Block {
        id: 2,
        hash: String::from("hash2"),
        previous_hash: String::from("prev_hash2"),
        timestamp: 234567890,
        data: String::from("Block 2 data"),
        nonce: 0,
    };

    let app = App {
        blocks: vec![block1, block2],
    };

    // JSON 직렬화 예제
    let serialized = serde_json::to_string(&app).unwrap();
    println!("Serialized: {}", serialized);

    // JSON 역직렬화 예제
    let deserialized: App = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
