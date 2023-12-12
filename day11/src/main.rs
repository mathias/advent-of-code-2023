use std::fs;

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

fn part_1(file_path: &str) -> u64 {
    let lines = parse_file(file_path);
    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut map: Vec<char> = vec![]; // will be width * height -- don't want to bother with default 
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            map.push(character);
        }
    }

    // find gaps
    let mut x_histogram: Vec<u64> = vec![0; width];
    let mut y_histogram: Vec<u64> = vec![0; height];

    for (i, character) in map.iter().enumerate() {
        let (x, y) = xy_from_map_idx(width, height, i);
        if *character == '#' {
            x_histogram[x] += 1;
            y_histogram[y] += 1;
        }
    }

    // expand where histogram 0
    let mut new_width = width.clone() + x_histogram.into_iter().fold(0, |acc, i| if i == 0 { acc + 1 } else { acc });
    let mut new_height = height.clone() + y_histogram.into_iter().fold(0, |acc, i| if i == 0 { acc + 1 } else { acc });

    let mut new_map = Vec<char> = vec!['.'; new_width * new_height];
    for (i, character) in map.iter().enumerate() {
        // check if we're inserting this col
        let (x, y) = xy_from_map_idx(height, width, i)
    }



    // build map of coordinates and connected locations
    // find S coordinates
    // A* to all points in map
    // Find highest number that can be reached through two sets of coordinates
    0
}

fn main() {
    let sample_1_file_path = "sample_input.txt";
    let input_file_path = "input.txt";
    let sample_1_expected: u64 = 374;

    assert!(xy_from_map_idx(140, 140, 1) == (1, 0), "Expected (1, 0) got {:?}", xy_from_map_idx(140,140,1));
    assert!(xy_from_map_idx(140, 140, 424) == (4, 3));

    println!("Part 1:");
    let sample_1_total = part_1(sample_1_file_path);
    assert!(sample_1_expected == sample_1_total, "score for sample 1 input must be {}, got {}", sample_1_expected, sample_1_total);
    println!("Sample 1 total: {}", sample_1_total);
    //let input_total = part_1(input_file_path);
    //println!("Input total: {}", input_total);
}
