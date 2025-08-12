use std::fs;

fn sort_vec(singles: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut sorted_vec = singles;
    sorted_vec.sort_by(|a, b| a.1.cmp(&b.1));
    sorted_vec
}

fn sum_vec(_vec_to_sum: Vec<i32>) -> i32{
    let _sum: i32 = _vec_to_sum.iter().sum();
    _sum
}

pub fn main() {
    let mut _values: Vec<String> = Vec::new();
    let input_file = "src/two_values/input_two";
    if let Ok(file_content) = fs::read_to_string(input_file) {
        _values = file_content.lines().map(String::from).collect();
    }
    let _numbers: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut _vec_to_sum: Vec<i32> = Vec::new();
    for current_word in _values.iter() {
        let mut singles = Vec::new();

        for number in _numbers.iter() {
            if let Some(begin) = current_word.find(number) {
                match number.to_string().as_str() {
                    "one" => singles.push((1, begin)),
                    "two" => singles.push((2, begin)),
                    "three" => singles.push((3, begin)),
                    "four" => singles.push((4, begin)),
                    "five" => singles.push((5, begin)),
                    "six" => singles.push((6, begin)),
                    "seven" => singles.push((7, begin)),
                    "eight" => singles.push((8, begin)),
                    "nine" => singles.push((9, begin)),
                    _ => {}
                }
            }
        }

        let sorted_vec = sort_vec(singles.clone());

        if let Some(_first) = sorted_vec.first(){
            if let Some(_last) = sorted_vec.last(){
                let _new_num = format!("{}{}", _first.0, _last.0);
                if !_new_num.is_empty(){
                    _vec_to_sum.push(_new_num.parse::<i32>().unwrap());
                }
            }
        }
    }
    let _sum = sum_vec(_vec_to_sum.clone());
    println!("SUM = {}", _sum);
}





