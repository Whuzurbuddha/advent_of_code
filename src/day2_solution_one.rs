use std::fs;

fn puzzle_input(input_file: &str) -> Vec<String> {
    if let Ok(file_content) = fs::read_to_string(input_file) {
        return file_content.lines().map(String::from).collect();
    }
    Vec::new()
}

pub fn main() {
    let input_file = "src/day2_solution_one_values/input";
    let values = puzzle_input(input_file);

    let mut possible_games: Vec<String> = Vec::new();

    for game_unfiltered in &values {
        let filtered_game = game_unfiltered.split(":").collect::<Vec<_>>()[1];
        let played_rounds: Vec<_> = filtered_game.split(";").collect();

        for played_games in played_rounds.iter() {
            let is_possible = played_games.split(',')
                .all(|round| {
                    let cube: Vec<_> = round.split_whitespace().collect();
                    let num = cube[0].parse::<i32>().unwrap_or(0);
                    let col = cube[1];

                    match col {
                        "blue" => num <= 14,
                        "green" => num <= 13,
                        "red" => num <= 12,
                        _ => false,
                    }
                });

            if is_possible {
                possible_games.push("POSSIBLE GAME".to_string());
            }
        }
    }
    println!("{}", possible_games.iter().count());
}
