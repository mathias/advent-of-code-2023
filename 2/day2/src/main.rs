use std::fs;

struct Game {
    red: i32,
    green: i32,
    blue: i32
}

const BAG : Game = Game { red: 12, green: 13, blue: 14 };

fn main() {
    let file_path : String = "input.txt".to_string();

    let mut possible_game_ids = vec![];
    possible_game_ids.push(0i32);

    for line in fs::read_to_string(file_path).unwrap().lines() {
        //input.push(line)
        // parse line into a game
        // determine if game can be possible with BAG
        // push onto possible_game_ids if possible
        let (first, rest) = line.split_once (":").unwrap();
        let (_game, id) = first.split_once(" ").unwrap();

        println!("Game #{}", id);
        let (mut red, mut green, mut blue) = (0i32, 0i32, 0i32);

        for draw in rest.split(";") {
            for value in draw.split(", ") {
                let (number_str, color) = value.trim().split_once(" ").unwrap();
                let number: i32 = number_str.parse().unwrap();

                match color {
                    "red" if number > red => red = number,
                    "green" if number > green => green = number,
                    "blue" if number > blue => blue = number,
                    _  => ()
                }
            }

        }
        println!("{} {} {}", red, green, blue);

        if red <= BAG.red && green <= BAG.green && blue <= BAG.blue {
            possible_game_ids.push(id.parse().unwrap());
        }

    }

    // add up possible_game_ids and print it:
    println!("{}", possible_game_ids.into_iter().sum::<i32>());
}
