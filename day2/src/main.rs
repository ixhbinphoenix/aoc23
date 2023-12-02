use std::fs;



fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = input.lines();

    let games = lines.map(|l| parse_to_game(l)).collect::<Vec<Game>>();

    // Part 1: 12 red, 13 green, 14 blue
    let mut x: u32 = 0;
    for game in games.clone() {
        if validate_game(&game) {
            x += game.id;
        }
    }
    println!("Part 1: {}", x);

    // Part 2: max(round.red) * max(round.green) * max(round.blue)
    let mut y: u32 = 0;
    for game in games {
        let mut required = Round { red: 0, green: 0, blue: 0 };
        for round in game.rounds {
            if round.red > required.red { required.red = round.red };
            if round.green > required.green { required.green = round.green };
            if round.blue > required.blue { required.blue = round.blue };
        };
        let power = required.red * required.green * required.blue;
        y += power;
    }
    println!("Part 2: {}", y);
}

fn validate_game(game: &Game) -> bool {
    for round in &game.rounds {
        if round.red > 12 || round.green > 13 || round.blue > 14 {
            return false;
        }
    }
    true
}

#[derive(Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>
}

#[derive(Clone)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_to_game(line: &str) -> Game {
    let mut first_split = line.split(":");
    let game_id: u32 = first_split.next().unwrap().split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
    let rounds_raw = first_split.next().unwrap().split(";");

    let mut rounds: Vec<Round> = Vec::new();

    for round_raw in rounds_raw {
        let mut round = Round { red: 0, blue: 0, green: 0 };
        for cubes in round_raw.split(",") {
            let mut a = cubes.trim().split(" ");
            let amount: u32 = a.next().unwrap().parse().unwrap();
            let color = a.next().unwrap();

            match color {
                "red" => {
                    round.red = amount;
                },
                "blue" => {
                    round.blue = amount;
                },
                "green" => {
                    round.green = amount;
                },
                _ => { panic!("Something went wrong parsing") }
            }
        }
        rounds.push(round);
    }

    Game { id: game_id, rounds }
}
