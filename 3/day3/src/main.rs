use std::fs;

// find full number for any found line and index
fn number_from(line: Vec<char>, index: usize) -> u32 {
    println!("{:?}", line);
    // numbers can be up to three digits long
    line[index].to_digit(10).unwrap()
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
            if ['$', '@', '+', '-', '*', '/', '&', '#'].contains(this_char) {
                // find any digit characters in the neighborhood:
                if lines[i-1][j-1].is_ascii_digit() { // upper left
                    valid_part_numbers.push(number_from(lines[i-1].clone(), j-1));
                } else if lines[i-1][j].is_ascii_digit() { // above
                    valid_part_numbers.push(number_from(lines[i-1].clone(), j));
                } else if lines[i-1][j+1].is_ascii_digit() { // upper right
                    valid_part_numbers.push(number_from(lines[i-1].clone(), j+1));
                } else if lines[i][j-1].is_ascii_digit() { // to left
                    valid_part_numbers.push(number_from(lines[i].clone(), j-1));
                } else if lines[i][j+1].is_ascii_digit() { // to right
                    valid_part_numbers.push(number_from(lines[i].clone(), j+1));
                } else if lines[i+1][j-1].is_ascii_digit() { // lower left
                    valid_part_numbers.push(number_from(lines[i+1].clone(), j-1));
                } else if lines[i+1][j].is_ascii_digit() { // below
                    valid_part_numbers.push(number_from(lines[i+1].clone(), j));
                } else if lines[i+1][j+1].is_ascii_digit() { // lower right
                    valid_part_numbers.push(number_from(lines[i+1].clone(), j+1));
                }
            }
        }
    }
    valid_part_numbers.into_iter().sum()
}

fn main() {
    let sample_file_path = "sample_input.txt";

    println!("{}", process_file(sample_file_path));
}
