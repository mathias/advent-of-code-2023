use std::fs;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\d").unwrap();
    //let file_path : String = "../sample_input.txt".to_string();
    let file_path : String = "../input.txt".to_string();
    let mut result = vec![];

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let caps : Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first_digit : i32 = caps.first().unwrap().parse().unwrap();
        let last_digit : i32 = caps.last().unwrap().parse().unwrap();

        result.push(first_digit * 10 + last_digit);
    }

    let sum : i32 = result.into_iter().reduce(|acc, e| acc + e).unwrap();

    println!("{}", sum);
}
