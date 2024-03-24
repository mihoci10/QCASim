use crate::sim::QCACell;

pub fn cells_serialize(cells: &Vec<QCACell>) -> String {
    return serde_json::to_string(cells).unwrap()
}

pub fn cells_deserialize(text: &String) -> Vec<QCACell> {
    return serde_json::from_str(text).unwrap()
}