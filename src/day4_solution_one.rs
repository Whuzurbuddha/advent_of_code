use std::fs;

fn puzzle_input(input_file: &str) -> Vec<String> {
    if let Ok(file_content) = fs::read_to_string(input_file) {
        return file_content.lines().map(String::from).collect();
    }
    Vec::new()
}

pub fn main(){
    let input_file = "src/day4_values/input";
    let _values = puzzle_input(input_file);

    for _card in _values.iter().enumerate(){
        let mut _winning_nums_per_card: Vec<&&str> = Vec::new();
        let _card_id = _card.1.split(':').collect::<Vec<_>>()[0];

        let _card_nums = _card.1
                            .split(':')
                            .collect::<Vec<_>>()[1]
                            .split('|')
                            .collect::<Vec<_>>()[0]
                            .split_whitespace()
                            .collect::<Vec<_>>();

        let _my_nums = _card.1
                            .split(':')
                            .collect::<Vec<_>>()[1]
                            .split('|')
                            .collect::<Vec<_>>()[1]
                            .split_whitespace()
                            .collect::<Vec<_>>();

        for (_num_index, _my_num) in _my_nums.iter().enumerate() {
            if _card_nums.contains(_my_num) {
                _winning_nums_per_card.push(_my_num);
            }
        }
        for (_index, _round) in _winning_nums_per_card.iter().enumerate(){
            print!("{_card_id} {:?} ", _round);
        }
        print!("\n");
    }
}