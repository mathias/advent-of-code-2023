use std::fs;
use regex::Regex;

fn main() {
    let sample_file_path : String = "../sample_input.txt".to_string();
    let file_path : String = "../input.txt".to_string();

    assert_eq!(process_file(sample_file_path), 281);

    println!("{}", process_file(file_path));
}

fn process_file(file_path: String) -> i32 {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut result = vec![];

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let caps : Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        println!("{:#?}", caps);
        let first_digit = parse_digit(caps.first().unwrap());
        let last_digit = parse_digit(caps.last().unwrap());

        result.push(first_digit * 10 + last_digit);
    }

    let sum : i32 = result.into_iter().reduce(|acc, e| acc + e).unwrap();

    return sum;
}
fn parse_digit(digit: &str) -> i32 {
    let maybe_digit = match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0
    };

    if maybe_digit > 0 {
        return maybe_digit
    } else {
        return digit.parse().unwrap()
    }
}
