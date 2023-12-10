use std::fs;

fn parse_map(file_path: &str) -> (String, Vec<(String, String, String)>) {
    let mut lines: Vec<String> = vec![];
    let instructions: String;
    let mut nodes: Vec<(String, String, String)> = vec![];

    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        lines.push(line.to_string());
    }

    let mut lines_iter = lines.iter();

    instructions = lines_iter.next().unwrap().to_string();

    // drop empty line
    lines_iter.next();

    for line in lines_iter {
        let (node_id, left_right) = line.split_once(" = ").unwrap();
        let cleaned_left_right = left_right.replace("(","").replace(")","");
        let (left, right) = cleaned_left_right.split_once(", ").unwrap();

        nodes.push((node_id.to_string(), left.to_string(), right.to_string()));
    }

    (instructions, nodes)
}

fn choose_next_node(instruction: char, current_node: String, nodes: Vec<(String, String, String)>) -> String {
    let next_node: String;
    let mut nodes_iter = nodes.clone().into_iter();

    // find node in nodes for current_node
    let this_node = nodes_iter.find(|(id, _, _)| *id == current_node).expect(&format!("Should always have a node to find, step {} id {}", instruction, current_node));

    // choose left or right based on instruction
    if instruction == 'L' {
        (_, next_node, _) = this_node;
    } else {
        (_, _, next_node) = this_node;
    }

    next_node
}

fn vec_of_instructions(instructions: String) -> Vec<char> {
    let mut vec_of_instructions: Vec<char> = vec![];
    for instruction in instructions.chars() {
        vec_of_instructions.push(instruction);
    }
    vec_of_instructions
}

fn part_1(file_path: &str) -> u64 {
    let (instructions, nodes) = parse_map(file_path);

    let mut current_node = "AAA".to_string();
    let mut steps: u64 = 0;

    for instruction in vec_of_instructions(instructions).iter().cloned().cycle() { // cycle until finished
        if current_node == "ZZZ".to_string() {
            return steps
        }

        current_node = choose_next_node(instruction, current_node.to_string(), nodes.clone());

        steps += 1;
    }
    return steps
}

fn ends_with(string: &str, ends_with: &str) -> bool {
    string.to_string().ends_with(ends_with)
}

fn part_2(file_path: &str) -> u64 {
    let (instructions, nodes) = parse_map(file_path);
    let mut steps: u64 = 0;

    let start_full_nodes: Vec<(String, String, String)> = nodes.clone().into_iter().filter(|(id, _, _)| ends_with(id, "A")).collect();
    let mut start_nodes: Vec<String> = vec![];
    for (node_loc, _, _) in start_full_nodes.into_iter() {
        start_nodes.push(node_loc);
    }
    println!("Found {} initial nodes to walk in parallel", start_nodes.len());
    let mut current_nodes = start_nodes.clone();

    //let mut final_state_met = false;
    let binding = vec_of_instructions(instructions);
    let instructions_iter = binding.iter().cloned().cycle();

    //while !final_state_met {
        //let instruction = instructions_iter.next().unwrap();

        //let mut next_nodes = vec![];

        //for node in &current_nodes {
             //choose next step for this node:
            //let next_node = choose_next_node(instruction, node.to_string(), nodes.clone());

            //next_nodes.push(next_node);
        //}
         //check if all current_nodes end in Z
        //if next_nodes.clone().into_iter().all(|x| ends_with(&x, "Z")) {
            //final_state_met = true;
        //}
        //current_nodes = next_nodes.clone();
        //steps += 1;
        //if steps % 1000 == 0 {
            //print!("+");
        //}
    //}
    let mut steps: Vec<u64> = vec![];
    for node in start_nodes.clone().into_iter() {
        let these_instructions =  instructions_iter.clone();
        let mut this_steps: u64 = 0;
        let mut current_node = node;

        for instruction in these_instructions { // cycle until finished
            if ends_with(&current_node, "Z") {
                break;
            }

            current_node = choose_next_node(instruction, current_node.to_string(), nodes.clone());

            this_steps += 1;
        }
        steps.push(this_steps);
    }
    //dbg!(steps);

    // then we need to LCM the steps array:
    //println!("");
    steps.iter().fold(1, |acc, b| lcm(acc, *b))
}

fn lcm(number: u64, other: u64) -> u64 {
    number * other / gcd(number, other)
}

fn gcd(number: u64, other: u64) -> u64 {
    let mut max = number;
    let mut min = other;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min
        }

        max = min;
        min = res;
    }
}

fn main() {
    let sample_file_path = "sample_input.txt";
    let sample_file_2_path = "sample_input_2.txt";
    let sample_file_3_path = "sample_input_3.txt";
    let input_file_path = "input.txt";
    let sample_1_steps: u64 = 2;
    let sample_2_steps: u64 = 6;

    println!("Part 1:");
    let sample_total = part_1(sample_file_path);
    assert!(sample_1_steps == sample_total, "score for sample input must be {}, got {}", sample_1_steps, sample_total);

    let sample_2_total = part_1(sample_file_2_path);
    assert!(sample_2_steps == sample_2_total, "score for sample input 2 must be {}, got {}", sample_2_steps, sample_2_total);

    println!("Sample 1 steps: {}, sample 2 steps: {}", sample_1_steps, sample_2_steps);
    println!("Full input: {}", part_1(input_file_path));


    println!("Part 2:");
    let sample_3_total = part_2(sample_file_3_path);
    let sample_3_steps: u64 = 6;
    assert!(sample_3_steps == sample_3_total, "score for part 2 sample input 3 must be {}, got {}", sample_3_steps, sample_3_total);

    println!("Sample 3 steps: {}", sample_3_steps);
    println!("Full input: {}", part_2(input_file_path));

}
