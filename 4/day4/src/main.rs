use std::fs;

fn string_to_vec_ints(string: String) -> Vec<u32> {
    return string.split(' ').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn process_file(file_path: &str) -> Vec<u32> {
    let mut matches = vec![];

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let (first, second) = line.split_once("|").unwrap();
        let (_game_id_str, winning_numbers_string) = first.split_once(":").unwrap();

        let winning_numbers = string_to_vec_ints(winning_numbers_string.to_string());
        let game_numbers = string_to_vec_ints(second.to_string());

        let mut score = 0;

        for number in game_numbers {
            if winning_numbers.contains(&number) {
                score += 1;
                //if score == 0 {
                    //score = 1;
                //} else {
                    //score = score * 2;
                //}
            }
        }
        matches.push(score);
    }
    matches
}

fn part_2(scores: Vec<u32>) -> usize {
    let scores_length: usize = scores.len();
    let range_len: usize;
    if scores_length > 0 {
        range_len = scores_length - 1;
    } else {
        range_len = 0;
    }

    let mut counts : Vec<usize> = vec![1; scores_length];

    for (i, val) in scores.iter().enumerate() {
        let matches: usize = val.clone() as usize;

        let end_range = if matches > range_len {
            range_len
        } else {
            matches + i + 1
        };

        for j in i+1..end_range {
            counts[j] += counts[i];
        }
        dbg!(counts.clone());
    }

    let total = counts.iter().sum();
    total
}

fn main() {
    let sample_file_path = "sample_input.txt";
    let input_path = "input.txt";
    let sample_score = 13;

    let sample_scores = process_file(sample_file_path);
    let sample_total: i32 = sample_scores.iter().filter(|x| **x > 0).map(|x| 2_i32.pow(*x - 1)).sum();
    assert!(sample_total == sample_score, "Score for sample input must be {}, got {}", sample_score, sample_total);

    let sample_part_2 = part_2(sample_scores);
    let sample_part_2_expected = 30;
    assert!(sample_part_2 == sample_part_2_expected, "Sample part 2: Expected {} got {}", sample_part_2_expected, sample_part_2);

    let input_scores: Vec<u32> = process_file(input_path);
    let part_1_total: i32 = input_scores.iter().filter(|x| **x > 0).map(|x| 2_i32.pow(*x - 1)).sum();
    println!("Day 4 part 1: {}", part_1_total);
    println!("debug: {:?}", input_scores);
    println!("Day 4 part 2: {}", part_2(input_scores));
}
