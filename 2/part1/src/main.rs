use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut num_safe: i32 = 0;

    contents.lines().for_each(| x | num_safe += is_safe(x));
    println!("{num_safe}");
}

fn is_safe(line: &str) -> i32 {
    let mut nums = line.split_whitespace();

    let mut prev_int: i32 = nums.nth(0).unwrap().parse::<i32>().unwrap_or(0);
    let mut dir: i32 = 0;
    let mut result: i32 = 1;
    nums.for_each(| x | {
        let value = x.parse::<i32>().unwrap_or(0);

        if dir == 0 {
            dir = prev_int - value
        }

        // println!("Dir: {dir} comparing {prev_int} to {value}");
        if (prev_int - value) == 0 || (prev_int - value).is_negative() != (dir).is_negative() || prev_int.abs_diff(value) > 3 {
            result = 0
        }

        prev_int = value;
    });

    result
}
