use std::fs;
use std::time::Instant;

fn parse_file(file_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        lines.push(line.to_string());
    }
    lines
}

fn calculate_solutions(springs: &Vec<char>, groups: &Vec<usize>) -> usize {
    if springs.is_empty() {
        if groups.is_empty() {
            return 1
        } else {
            return 0
        }
    }

    match springs[0] {
        '.' => calculate_solutions(&springs[1..].to_vec(), &groups),
        '#' => calculate_hash_solutions(&springs, &groups),
        '?' => calculate_solutions(&springs[1..].to_vec(), &groups) + calculate_hash_solutions(&springs, &groups),
        _ => panic!("At the disco!")
    }
}

fn calculate_hash_solutions(springs: &Vec<char>, groups: &Vec<usize>) -> usize {
    if groups.is_empty() {
        return 0
    }

    let x = groups[0];
    if springs.len() < x {
        return 0
    }


    for i in 0..x {
        if springs[i] == '.' {
            return 0
        }
    }

    if springs.len() == x {
        if groups.len() == 1 {
            return 1
        } else {
            return 0
        }
    }

    if springs[x] == '#' {
        return 0
    }
    let result = calculate_solutions(&springs[(x+1)..].to_vec(), &groups[1..].to_vec());
    result
}

fn process_row(line: String) -> usize {
    let (first, last) = line.split_once(" ").unwrap();
    let springs: Vec<char> = first.chars().collect();

    let split_groups_strs: Vec<&str> = last.split(",").collect();
    let split_groups: Vec<usize> = split_groups_strs.into_iter().map(|x| x.parse().unwrap()).collect();

    let result = calculate_solutions(&springs, &split_groups);

    result
}

fn part_1(file_path: &str) -> usize {
    let lines = parse_file(file_path);
    let mut sum = 0;

    for line in lines {
        sum += process_row(line.clone());
    }

    sum
}

fn part_2(file_path: &str) -> usize {
    let lines = parse_file(file_path);
    let mut sum = 0;

    for mut line in lines {
        // modify each line thusly:
        // replace the list of springs with five copies of itself, separated by ?
        // replace the list of groups with five copies of itself, separated by ,
        let (first, last) = line.split_once(" ").unwrap();

        let new_springs: String = (0..5).map(|_| first.chars().collect()).collect::<Vec<String>>().join("?");
        let new_groups: String = (0..5).map(|_| last.chars().collect()).collect::<Vec<String>>().join(",");

        line = vec![new_springs, new_groups].join(" ");
        dbg!(&line);

        sum += process_row(line.clone());
    }

    sum
}


fn process_input_1(name: &str, file_path: &str) -> usize {
    let start = Instant::now();
    let total = part_1(file_path);
    let duration = start.elapsed();
    println!("Time elapsed in {} is: {:?}", name, duration);

    total
}

fn process_input_2(name: &str, file_path: &str) -> usize {
    let start = Instant::now();
    let total = part_2(file_path);
    let duration = start.elapsed();
    println!("Time elapsed in {} is: {:?}", name, duration);

    total
}

fn main() {
    let sample_1_file_path = "sample_input.txt";
    let input_file_path = "input.txt";
    let sample_1_expected: usize = 21;

    println!("Part 1:");
    let sample_1_total = process_input_1("Sample 1", sample_1_file_path);
    assert!(sample_1_expected == sample_1_total, "score for sample 1 input must be {}, got {}", sample_1_expected, sample_1_total);
    println!("Sample 1 total: {}", sample_1_total);
    println!("Input total: {}", process_input_1("Input", input_file_path));

    println!("Part 2:");
    let sample_part_2_expected: usize = 525152;
    let sample_part_2_total = process_input_2("Sample part 2", sample_1_file_path);
    assert!(sample_part_2_expected == sample_part_2_total, "score for part 2 sample input must be {}, got {}", sample_part_2_expected, sample_part_2_total);
    println!("Part 2 sample input total: {}", sample_part_2_total);
    //println!("Input total: {}", process_input_2("Input", input_file_path));
}
