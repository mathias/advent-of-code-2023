use std::fs;
use core::fmt;

#[derive(Clone)]
struct Race {
    time: u64,
    distance: u64
}

impl fmt::Debug for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Race")
            .field("time", &self.time)
            .field("distance", &self.distance)
            .finish()
    }
}

fn score_races(races: Vec<Race>) -> u64 {
    let mut wins: u64 = 1;

    for race in races {
        let mut race_wins: u64 = 0;
        for time in 0..=race.time {
            if win_for_time(race.clone(), time) {
                race_wins += 1;
            }
        }
        wins *= race_wins;
    }

    wins
}

fn score_part_1(file_path: &str) -> u64 {
    let mut lines: Vec<Vec<String>> = vec![];
    let mut races : Vec<Race> = vec![];

    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        let mut line: Vec<String> = line.split_ascii_whitespace().map(|x| x.to_string()).collect();

        // first position is labels and can be skipped
        line.remove(0);

        lines.push(line);
    }

    assert!(lines.len() == 2);

    let mut race: Race;

    let races_count = lines[0].len();

    for i in 0..races_count {
        race = Race {
            time: lines[0][i].parse().unwrap_or(0),
            distance: lines[1][i].parse().unwrap_or(0)
        };

        races.push(race);
    }

    score_races(races.clone())
}

fn score_part_2(file_path: &str) -> u64 {
    let mut lines: Vec<String> = vec![];
    let mut races : Vec<Race> = vec![];

    let binding = fs::read_to_string(file_path).unwrap();
    for line in binding.lines() {
        let (_labels, split_line) = line.split_once(":").unwrap();

        // whitespace can be removed
        let result_line = &split_line.to_string().replace(" ", "");

        lines.push(result_line.to_string());
    }

    assert!(lines.len() == 2);

    let race = Race {
        time: lines[0].parse().unwrap_or(0),
        distance: lines[1].parse().unwrap_or(0)
    };
    races.push(race);

    score_races(races.clone())
}

fn win_for_time(race: Race, time_held: u64) -> bool {
    // race.time , race.distance
    let speed = time_held.clone();
    let time_remaining = race.time - time_held;

    if time_remaining <= 0 { return false }

    time_remaining * speed > race.distance

}



fn main() {
    let sample_file_path = "sample_input.txt";
    let input_file_path = "input.txt";
    let sample_score = 288;

    println!("Part 1:");
    let sample_total = score_part_1(sample_file_path);
    assert!(sample_score == sample_total, "Score for sample input must be {}, got {}", sample_score, sample_total);
    println!("Wins for sample input: {}", sample_total);

    let input_total = score_part_1(input_file_path);
    println!("Wins for input: {}", input_total);

    println!("Part 2:");
    let sample_part_2 = 71503;
    assert!(sample_part_2 == score_part_2(sample_file_path), "Score for part 2 sample must be {}", sample_part_2);
    println!("Wins for input part 2: {}", score_part_2(input_file_path));
}
