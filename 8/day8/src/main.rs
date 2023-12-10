use std::fs;

fn parse_input(file_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        lines.push(line.to_string());
    }

    lines
}

fn main() {
    let sample_file_path = "sample_input.txt";
    //let input_file_path = "input.txt";
    //let sample_score = 6440;

    println!("Part 1:");
    //let sample_total = parse_input(sample_file_path);
    //assert!(sample_score == sample_total, "score for sample input must be {}, got {}", sample_score, sample_total);
}
