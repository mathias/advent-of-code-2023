use std::env;
use std::fs;

fn main() {
    let file_path : String = "../input.txt".to_string();
    let mut result = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        result.push(line.to_string());
    }

    print!("{:#?}", result);
}
