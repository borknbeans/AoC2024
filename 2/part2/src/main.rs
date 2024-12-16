use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut num_safe = 0;

    contents.lines().for_each(|x| {
        let mut nums: Vec<i32> = Vec::new();
        
         x.split_whitespace().for_each(|n| {
            nums.push(n.parse::<i32>().unwrap_or(0));
        });
        num_safe += (is_safe_check_all(nums)) as i32;
    });

    println!("Num safe: {num_safe}");
}

fn is_safe_check_all(nums: Vec<i32>) -> bool {
    let mut valid = false;

    let nums_copy = nums.clone();

    let mut index = 0;
    nums.into_iter().for_each(|x| {
        let mut nums_exclusive = nums_copy.clone();
        nums_exclusive.remove(index);
        index += 1;

        if is_safe(nums_exclusive) {
            valid = true;
        }
    });

    if is_safe(nums_copy) {
        valid = true;
    }

    valid
}

fn is_safe(nums: Vec<i32>) -> bool {
    let mut prev_num = nums[0];
    let mut dir = 0;

    let mut valid = true;

    nums.into_iter().skip(1).for_each(|n| {
        if dir == 0 {
            dir = prev_num - n;
        }

        let current_dir = prev_num - n;

        if prev_num == n || dir.is_negative() != current_dir.is_negative() || prev_num.abs_diff(n) > 3 {
            valid = false;
        }

        prev_num = n;
    });

    valid
}
