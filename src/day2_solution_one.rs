use std::fs;

fn puzzle_input(input_file: &str) -> Vec<String> {
    if let Ok(file_content) = fs::read_to_string(input_file) {
        return file_content.lines().map(String::from).collect();
    }
    Vec::new()
}
fn sum(_IDS: Vec<&str>) -> i32{
    let mut _digits_vec: Vec<i32> = Vec::new();
    let mut _sum = 0;
    for _items in _IDS.iter().enumerate(){
        _sum += _items.1.split_whitespace().collect::<Vec<_>>()[1].parse::<i32>().unwrap_or_default();
    };
    _sum
}
pub fn main() {
    let input_file = "src/day2_solution_one_values/input";
    let values = puzzle_input(input_file);


    let mut possible_games: Vec<&str> = Vec::new();
    for game_unfiltered in &values {
        let _ID = game_unfiltered.split(":").collect::<Vec<_>>()[0];
        let filtered_game = game_unfiltered.split(":").collect::<Vec<_>>()[1];
        let played_rounds: Vec<_> = filtered_game.split(";").collect();

        let mut _possible_round = Vec::new();
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
                _possible_round.push("POSSIBLE".to_string());
            }else{
                _possible_round.push("IMPOSSIBLE".to_string());
            }
        }
        if _possible_round.iter().find(|&x| x == "IMPOSSIBLE").is_some() {
            _possible_round.clear();
        }else{
            possible_games.push(_ID);
        }
    }
    let _sum =  sum(possible_games);
    println!("SUM: {}", _sum);
}
