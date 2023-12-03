use std::fs;

pub fn main() {
    let mut _values: Vec<String> = Vec::new();
    let input_file = "src/two_values/input_two";
    if let Ok(file_content) = fs::read_to_string(input_file) {
        _values = file_content.lines().map(String::from).collect();
    }
    let mut _word_vec = Vec::new();
    let _numbers: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for _word in _values.iter() {
        let mut _singles = Vec::new();
        let current_word = _word.clone();

        for _number in _numbers.iter() {
            if current_word.find(_number).is_some(){
                match _number.to_string().as_str() {
                    "one"   => _singles.push(1),
                    "two"   => _singles.push(2),
                    "three" => _singles.push(3),
                    "four"  => _singles.push(4),
                    "five"  => _singles.push(5),
                    "six"   => _singles.push(6),
                    "seven" => _singles.push(7),
                    "eight" => _singles.push(8),
                    "nine"  => _singles.push(9),
                    _=> {}
                }
            }
        }
        _word_vec.push(_singles);
    }
    for(_index, _number) in _word_vec.iter().enumerate(){
        println!("{:?}", _number);
    }
}
