use std::fs;
fn find_next_to(_values: Vec<String>) -> Vec<String>{
    let mut _num_next_to_symbol = Vec::new();
    for _i in 0.._values.len() {
        let mut chars = _values[_i].chars().peekable();
        while let Some(_j) = chars.next() {
            if _j.is_ascii_punctuation() {
                let mut _number = String::new();
                let mut _start = _j;
                while let Some(_next_char) = chars.peek() {
                    if _next_char.is_digit(10) {
                        _number.push(*_next_char);
                        chars.next();
                    } else {
                        let _end = *_next_char;
                        if _start == '.' && _end == '.' {
                            _number.clear();
                        }
                        break;
                    }
                }
                if !_number.is_empty() {
                    _num_next_to_symbol.push(_number.clone());
                }
            }
        }
    }
    _num_next_to_symbol
}

fn find_all_number(_values: Vec<String>) -> Vec<String>{
    let mut _all_numbers = Vec::new();
    for _j in 0.._values.len() {
        let mut chars = _values[_j].chars().peekable();
        while let Some(_k) = chars.next() {
            if _k.is_ascii_punctuation(){
                let mut _number = String::new();
                let mut _start = _k;
                while let Some(_next_char) = chars.peek() {
                    if _next_char.is_digit(10) {
                        _number.push(*_next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !_number.is_empty(){
                    _all_numbers.push(_number.clone());
                }
            }
        }
    }
    _all_numbers
}
const DISTANCE_THRESHOLD: usize = 140;

fn find_diagonal(target_numbers: Vec<String>, input_strings: Vec<String>) -> Vec<String> {
    let symbols: [&str; 8] = ["+", "-", "/", "=", "%", "@", "$", "*"];
    let numbers_diagonal_to = Vec::new();

    for number in target_numbers {
        for symbol in &symbols {
            for value_string in &input_strings {
                if let Some(start) = value_string.find(&number){
                     let _start = start + 1;
                    if let Some(next_symbol) = value_string.find(symbol) {
                        let _distance = next_symbol - _start;
                        println!("{}", _start);
                        println!("{}", next_symbol);
                        println!("{}", _distance);
                    }
                }

            }
        }
    }

    numbers_diagonal_to
}


fn sum_of_vec(_vector: Vec<String>) -> i32{
    let mut _num_vector = Vec::new();
    for i in 0.._vector.len(){
        let _num = _vector[i].parse::<i32>().unwrap();
        _num_vector.push(_num);
    }
    let _sum = _num_vector.iter().sum();
    _sum
}
pub fn main() {
    let mut _values: Vec<String> = Vec::new();
    let input_file = "src/three_values/input";
    if let Ok(file_content) = fs::read_to_string(input_file) {
        _values = file_content.lines().map(String::from).collect();
    }

    /*let _num_next_to_symbol = find_next_to(_values);
    let _next_to_sum = sum_of_vec(_num_next_to_symbol);*/

    let _all_numbers = find_all_number(_values.clone());
    let _num_with_diagonal_symbol = find_diagonal(_all_numbers, _values.clone());
    //println!("{:?}", _num_with_diagonal_symbol);
    //let _sum_diagonal = sum_of_vec(_num_with_diagonal_symbol);
    println!("{:?}", _num_with_diagonal_symbol);
}
