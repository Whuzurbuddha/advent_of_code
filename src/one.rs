fn find_digits(_value: Vec<&str>) -> Vec<i32> {
    let mut _final_value: Vec<i32> = Vec::new();
    for (_index, _item) in _value.iter().enumerate() {
        let combined: i32 = _item
            .chars()
            .filter_map(|c| c.to_digit(10).map(|digit| digit as i32))
            .fold(0, |acc, digit| acc * 10 + digit);

        let last_digit = combined % 10;

        if combined < 10 {
            let first_digit = combined % 10;
            _final_value.push(first_digit * 10 + first_digit);
        } else {
            let mut divisor = 1;
            while combined / divisor >= 10 {
                divisor *= 10;
            }

            let first_digit = combined / divisor;
            _final_value.push(first_digit * 10 + last_digit);
        }
    }
    _final_value
}

fn sum_of_digits(_digits: Vec<i32>) -> i32{
    let _sum = _digits.iter().sum();
    return _sum;
}

pub fn main() {
    let _calibration_value: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let _digits = find_digits(_calibration_value);
    let _sum = sum_of_digits(_digits);
    println!("{}", _sum);
}
