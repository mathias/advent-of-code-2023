use std::fs;
use regex::Regex;

// find full number for any found line and index
fn number_from(line: Vec<char>, index: usize) -> u32 {
    dbg!(line.clone(), index);
    let (start_slice, end_slice);
    let end_of_line = line.len();

    if index > 4 {
        start_slice = index - 4;
    } else {
        start_slice = 0;
    }

    if index + 4 <= end_of_line { // 4 and not 3 because slice range end is not inclusive
        end_slice = index + 4;
    } else {
        end_slice = end_of_line;
    }

    let numbers = &line[start_slice..end_slice].into_iter().collect::<String>();
    let re = Regex::new(r"(\d{1,3})").unwrap();
    let matching_digits = re.find_iter(numbers);

    matching_digits.map(|x| {
        let number = x.as_str().parse::<u32>().unwrap();
        dbg!("{}", number);
        number
    }).sum()
}

fn process_file(file_path: &str) -> u32 {
    let mut lines: Vec<Vec<char>> = vec![];
    let mut valid_part_numbers: Vec<u32> = vec![];
    valid_part_numbers.push(0);

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let this_chars = line.chars().collect();
        lines.push(this_chars);
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, this_char) in line.iter().enumerate() {
            // find special characters
            if ['@', '#', '$', '%', '^', '&', '*', '/', '-', '+', '='].contains(this_char) {
                dbg!("found symbol in", line.clone());
                // find any digit characters in the neighborhood:
                if lines[i-1][j-1].is_ascii_digit() { // upper left
                    valid_part_numbers.push(number_from(lines[i-1].clone(), j-1));
                }
                if lines[i-1][j].is_ascii_digit() { // above
                    valid_part_numbers.push(number_from(lines[i-1].clone(), j));
                }
                if lines[i-1][j+1].is_ascii_digit() { // upper right
                    valid_part_numbers.push(number_from(lines[i-1].clone(), j+1));
                }
                if lines[i][j-1].is_ascii_digit() { // to left
                    valid_part_numbers.push(number_from(lines[i].clone(), j-1));
                }
                if lines[i][j+1].is_ascii_digit() { // to right
                    valid_part_numbers.push(number_from(lines[i].clone(), j+1));
                }
                if lines[i+1][j-1].is_ascii_digit() { // lower left
                    valid_part_numbers.push(number_from(lines[i+1].clone(), j-1));
                }
                if lines[i+1][j].is_ascii_digit() { // below
                    valid_part_numbers.push(number_from(lines[i+1].clone(), j));
                }
                if lines[i+1][j+1].is_ascii_digit() { // lower right
                    valid_part_numbers.push(number_from(lines[i+1].clone(), j+1));
                }
            }
        }
    }

    valid_part_numbers.into_iter().sum()
}

fn main() {
    let sample_file_path = "sample_input.txt";
    //let input_path = "input.txt";

    let sample_total = 4361;
    dbg!("{}", process_file(sample_file_path));
    assert!(process_file(sample_file_path) == sample_total, "Sum for sample input must be {}", sample_total);

    //println!("{}", process_file(input_path));
}
