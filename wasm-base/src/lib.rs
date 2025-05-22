use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessedItem {
    pub id: u32,
    pub processed: String
}

#[wasm_bindgen]
pub fn process_data(input: &str) -> String {
    let count: u32 = input.parse().unwrap_or(0);
    let result: Vec<ProcessedItem> = (0..count).map(|i| ProcessedItem {
        id: i,
        processed: format!("WASM_{}", i)
    }).collect();
    
    serde_json::to_string(&result).unwrap()
}
