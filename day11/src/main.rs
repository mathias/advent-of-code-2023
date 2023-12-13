use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn parse_file(file_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        lines.push(line.to_string());
    }
    lines
}

fn xy_from_map_idx(width: usize, height: usize, map_idx: usize) -> (usize, usize) {
    let x: usize = map_idx % width;
    let y: usize = map_idx / height;
    (x, y)
}

fn new_star_tuples(map: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    // find new star locations and save them as tuples:
    let mut new_star_locs = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if character == &'#' {
                new_star_locs.push((x, y));
            }
        }
    }
    new_star_locs
}

fn expand_map(factor: usize, lines: Vec<String>) -> Vec<Vec<char>> {
    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut oned_map: Vec<char> = vec![]; // will be width * height -- don't want to bother with default
    let mut mappy: Vec<Vec<char>> = vec![];

    for line in lines.iter() {
        let mut chars: Vec<char> = vec![];

        for character in line.chars() {
            oned_map.push(character);
            chars.push(character);
        }
        mappy.push(chars);
    }

    // find gaps
    let mut x_histogram: Vec<usize> = vec![0; width];
    let mut y_histogram: Vec<usize> = vec![0; height];

    for (i, character) in oned_map.iter().enumerate() {
        let (x, y) = xy_from_map_idx(width, height, i);
        if *character == '#' {
            x_histogram[x] += 1;
            y_histogram[y] += 1;
        }
    }

    // expand where histogram 0
    let new_width = width.clone() + x_histogram.clone().into_iter().fold(0, |acc, i| if i == 0 { acc + factor } else { acc });
    let new_height = height.clone() + y_histogram.clone().into_iter().fold(0, |acc, i| if i == 0 { acc + factor } else { acc });

    let mut new_map: Vec<Vec<char>> = vec![vec!['.'; new_width]; new_height];

    let mut y_offset_for_writing = 0;
    for (y, row) in mappy.iter().enumerate() {
        if y_histogram[y] == 0 {
            // We only need skip writing the y in writes below, since the default
            // character is '.'
            y_offset_for_writing += factor;
        }
        let mut x_offset_for_writing = 0;
        for (x, character) in row.iter().enumerate() {
            if x_histogram[x] == 0 {
                x_offset_for_writing += factor;
            } else {
                new_map[y+y_offset_for_writing][x+x_offset_for_writing] = character.clone();
            }
        }
    }

    new_map
}

fn star_manhatten_distances(star_locs: Vec<(usize, usize)>) -> Vec<usize> {
    let mut star_pairs = vec![];
    for pair in star_locs.into_iter().combinations(2) {
        star_pairs.push((pair[0], pair[1]));
    }

    star_pairs.into_iter().map(manhatten_distance).collect::<Vec<usize>>()
}

fn manhatten_distance(star_pair: ((usize, usize), (usize, usize))) -> usize {
    let (first, second) = star_pair;
    let (x1, y1) = first;
    let (x2, y2) = second;

    let (new_x, new_y);

    if x1 > x2 { new_x = x1-x2; } else { new_x = x2-x1; }
    if y1 > y2 { new_y = y1-y2; } else { new_y = y2-y1; }
    new_x + new_y
}

fn part_1(file_path: &str) -> u64 {
    let lines = parse_file(file_path);

    let factor = 1;
    let new_map = expand_map(factor, lines);

    let new_star_locs = new_star_tuples(new_map);

    // take each pair of stars, looks like I need to learn to use itertools:
    let distances = star_manhatten_distances(new_star_locs);
    distances.into_iter().sum::<usize>() as u64
}

fn part_2(file_path: &str) -> u64 {
    let lines = parse_file(file_path);

    let factor = 1000000;
    let new_map = expand_map(factor, lines);

    let new_star_locs = new_star_tuples(new_map);

    // take each pair of stars, looks like I need to learn to use itertools:
    let distances = star_manhatten_distances(new_star_locs);
    distances.into_iter().sum::<usize>() as u64
}

fn process_input_1(name: &str, file_path: &str) -> u64 {
    let start = Instant::now();
    let total = part_1(file_path);
    let duration = start.elapsed();
    println!("Time elapsed in {} is: {:?}", name, duration);

    total
}

fn process_input_2(name: &str, file_path: &str) -> u64 {
    let start = Instant::now();
    let total = part_2(file_path);
    let duration = start.elapsed();
    println!("Time elapsed in {} is: {:?}", name, duration);

    total
}
fn main() {
    let sample_1_file_path = "sample_input.txt";
    let input_file_path = "input.txt";
    let sample_1_expected: u64 = 374;

    assert!(xy_from_map_idx(140, 140, 1) == (1, 0), "Expected (1, 0) got {:?}", xy_from_map_idx(140,140,1));
    assert!(xy_from_map_idx(140, 140, 424) == (4, 3));

    println!("Part 1:");
    let sample_1_total = process_input_1("Sample", sample_1_file_path);
    assert!(sample_1_expected == sample_1_total, "score for sample 1 input must be {}, got {}", sample_1_expected, sample_1_total);
    println!("Sample 1 total: {}", sample_1_total);
    let input_total = process_input_1("Actual input", input_file_path);
    println!("Input total: {}", input_total);

    println!("Part 2!");
    //let sample_part_2_expected = 
    let input_part_2_total = process_input_2("Actual input", input_file_path);
    println!("Input total: {}", input_part_2_total);
}
