use std::fs;
fn puzzle_input(_input_file: &str) -> Vec<String>{
    let mut _values: Vec<String> = Vec::new();
    if let Ok(file_content) = fs::read_to_string(_input_file) {
        _values = file_content.lines().map(String::from).collect();
    }
    _values
}

fn sum_vec(_vec_to_sum: Vec<i32>) -> i32{
    let _sum: i32 = _vec_to_sum.iter().sum();
    _sum
}

pub fn main(){
    let _input_file = "src/day2_solution_one_values/input";
    let _values = puzzle_input(_input_file);
    let mut _possible_games: Vec<i32> = Vec::new();
    for (_index, _game_unfiltered) in _values.iter().enumerate() {
        let _filtered_game = _game_unfiltered.split(":").collect::<Vec<_>>()[1];
        let mut _possible_rounds = Vec::new();

        let _played_rounds: Vec<_> = _filtered_game.split(";").collect();
        for _played_round in _played_rounds {
            let mut _blue = true;
            let mut _green = true;
            let mut _red = true;

            let _single_rounds: Vec<_> = _played_round.split(",").collect();
            for _single_round in _single_rounds {
                let _num = _single_round.split_whitespace().collect::<Vec<_>>()[0].parse::<i32>();
                let _color = _single_round.split_whitespace().collect::<Vec<_>>()[1];

                match _color.to_string().as_str() {
                    "blue" => {
                        if let Ok(value) = _num {
                            if value > 14 {
                                _blue = false;
                            }
                        }
                    }
                    "green" => {
                        if let Ok(value) = _num {
                            if value > 13 {
                                _green = false;
                            }
                        }
                    }
                    "red" => {
                        if let Ok(value) = _num {
                            if value > 12 {
                                _red = false;
                            }
                        }
                    }
                    _ => {}
                }
            }

            if _blue && _green && _red {
                _possible_rounds.push("possible");
            }else{
                _possible_rounds.push("impossible");
            }
        }
        let mut _possible_count: i32 = 0;
        let mut _impossible_count: i32 = 0;
        for (_index, _rounds) in _possible_rounds.iter().enumerate(){
            match _rounds.to_string().as_str() {
                "possible" => _possible_count += 1,
                "impossible" => _impossible_count += 1,
                _=> {}
            }
        }
       _possible_games.push(_possible_count);
    }
    let _sum = sum_vec(_possible_games);
    println!("SUM OF ALL POSSIBLE GAMES: {}", _sum.to_string());
}