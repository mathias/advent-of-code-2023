use std::fs;


fn string_to_vec_ints(string: String) -> Vec<i32> {
    return string.split(' ').map(|s| s.trim())     // (2)
        .filter(|s| !s.is_empty())        // (3)
        .map(|s| s.parse().unwrap())      // (4)
        .collect()
}
fn process_file(file_path: &str) -> i32 {
    let mut scores = vec![];
    scores.push(0);

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let (first, second) = line.split_once("|").unwrap();
        let (_game_id_str, winning_numbers_string) = first.split_once(":").unwrap();

        let winning_numbers = string_to_vec_ints(winning_numbers_string.to_string());
        let game_numbers = string_to_vec_ints(second.to_string());

        let mut score = 0;

        for number in game_numbers {
            if winning_numbers.contains(&number) {
                if score == 0 {
                    score = 1;
                } else {
                    score = score * 2;
                }
            }
        }
        scores.push(score);
    }

    scores.iter().sum()
}

fn main() {
    let sample_file_path = "sample_input.txt";
    let input_path = "input.txt";

    let sample_score = 13;

    dbg!("{}", process_file(sample_file_path));
    assert!(process_file(sample_file_path) == sample_score, "Score for sample input must be {}", sample_score);

    println!("{}", process_file(input_path));
}
