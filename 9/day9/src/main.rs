use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<i64>> {
    let mut lines: Vec<Vec<i64>> = vec![];
    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        //lines.push(line.to_string());
        let parsed_line: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut sequence: Vec<i64> = vec![];
        for s in parsed_line.into_iter() {
            sequence.push(s.parse().unwrap());
        }
        lines.push(sequence);
    }

    lines
}

fn diff_pairs(line: Vec<i64>) -> Vec<i64> {
    let mut diffs: Vec<i64> = vec![];

    for slice in line.windows(2) {
        let [a, b] = slice else { todo!() };
        diffs.push(b - a);
    }
    diffs
}

fn part_1(file_path: &str) -> i64 {
    let lines = parse_file(file_path);
    let mut seqs_new_values: Vec<i64> = vec![0];

    for line in lines.into_iter() {
        let mut diff = line.clone();
        let mut diffs_triangle = vec![];
        diffs_triangle.push(diff.clone());

        while !(diff.iter().all(|x| *x == 0 )) {
            diff = diff_pairs(diff);
            diffs_triangle.push(diff.clone());
        }
        //dbg!(diffs_triangle);

        // take diff triangle and invert it
        diffs_triangle.reverse();
        let mut new_triangle: Vec<Vec<i64>> = vec![];
        let mut value_below = 0;

        for line in diffs_triangle {
            let mut new_line: Vec<i64>;
            let mut new_last = 0;
            // skip zeroes
            if !line.iter().all(|x| *x == 0) {
                let last_number = line.last().unwrap();
                new_last = value_below + last_number;
            } else {
                new_last = 0;
            }
            new_line = line.clone();
            new_line.push(new_last);
            new_triangle.push(new_line);
            value_below = new_last;
        }

        //dbg!(new_triangle.clone());


        // get final new number and push it on to seqs_new_values
        let new_last = new_triangle.last().unwrap().last().unwrap();
        seqs_new_values.push(*new_last);
    }


    return seqs_new_values.into_iter().sum()
}

//fn ends_with(string: &str, ends_with: &str) -> bool {
    //string.to_string().ends_with(ends_with)
//}

fn main() {
    let sample_file_path = "sample_input.txt";
    let input_file_path = "input.txt";
    let sample_1_sum: i64 = 114;

    println!("Part 1:");
    let sample_total = part_1(sample_file_path);
    assert!(sample_1_sum == sample_total, "score for sample input must be {}, got {}", sample_1_sum, sample_total);

    println!("Sample 1 steps: {}", sample_1_sum);
    println!("Full input: {}", part_1(input_file_path));

    //println!("Part 2:");

    //println!("Full input: {}", part_2(input_file_path));

}
