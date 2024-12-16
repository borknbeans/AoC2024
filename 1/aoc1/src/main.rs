use std::fs;

fn main() {
    read_file_into_arrays("input.txt");
}

fn read_file_into_arrays(filename: &str) {
    let contents = fs::read_to_string(filename)
    .expect("Unable to read file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let (left_int, right_int) = split_line_into_ints(line);
        // println!("{left_int} {right_int}");
        left.push(left_int);
        right.push(right_int);
    }

    left.sort();
    right.sort();

    //let diff = calculate_difference_in_values(left, right);
    //println!("Total difference: {diff}");

    let calc = calculate_left_list_appearances(left, right);
    println!("Calculated value: {calc}");
}

fn split_line_into_ints(line: &str) -> (i32, i32) {
    let mut left_int: i32 = 0;
    let mut right_int: i32 = 0;

    let mut i = 0;
    for s in line.split(" ") {
        let parsed_int = s.parse::<i32>();

        match parsed_int {
            Ok(num) => {
                if i == 0 {
                    left_int = num;
                } else {
                    right_int = num;
                }
            },
            Err(_) => (),
        };

        i += 1;
    }

    (left_int, right_int)
}

fn calculate_difference_in_values(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut diff: i32 = 0;

    for n in 0..left.len() {
        let left_value = left[n];
        let right_value = right[n];

        diff += (right_value - left_value).abs();
    }

    diff
}

fn calculate_left_list_appearances(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut output: i32 = 0;

    for num in left {
        let count = count_appearances_in_list(num, &right);

        output += num * count;
    }

    output
}

fn count_appearances_in_list(num: i32, list: &Vec<i32>) -> i32 {
    let mut count = 0;

    for n in list {
        if *n == num {
            count += 1;
        }
    }

    count
}