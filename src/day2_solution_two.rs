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

    let mut _color_products = Vec::new();

    let mut _highest_blue = Vec::new();
    let mut _highest_green = Vec::new();
    let mut _highest_red  = Vec::new();

    for game_unfiltered in &values {
        let parts: Vec<&str> = game_unfiltered.split(":").collect();
        let filtered_game = parts[1].trim();
        let played_rounds: Vec<&str> = filtered_game.split(";").collect();

        let mut _all_blue = Vec::new();
        let mut _all_red = Vec::new();
        let mut _all_green = Vec::new();

        let _height = played_rounds.iter().all(|played_games| {

            let _get_numbers = played_games.split(',').all(|round| {
                let cube: Vec<_> = round.split_whitespace().collect();
                let num = cube[0].parse::<i32>().unwrap_or(0);
                let col = cube[1];

                match col {
                    "blue" => _all_blue.push(num),
                    "green" => _all_green.push(num),
                    "red" => _all_red.push(num),
                    _ => {},
                }
                true
            });

            _get_numbers
        });
        _highest_blue.push(_all_blue.iter().fold(std::i32::MIN, |a,b| a.max(*b)));
        _highest_green.push(_all_green.iter().fold(std::i32::MIN, |a,b| a.max(*b)));
        _highest_red.push(_all_red.iter().fold(std::i32::MIN, |a,b| a.max(*b)));
    }
        for _i in 0.._highest_red.len(){
            let _color_product = _highest_blue[_i] * _highest_green[_i] * _highest_red[_i];
            _color_products.push(_color_product);
        }
    let _sum: i32 = _color_products.iter().sum();
    println!("{}", _sum);
}
