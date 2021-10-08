use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub value: String,
    pub offset: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProduceRequest {
    pub record: Record
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProduceResponse {
    pub offset: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumeRequest {
    pub offset: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumeResponse {
    pub record: Record
}