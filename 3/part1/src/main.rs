use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut output: i32 = 0;

    contents.lines().for_each(|x| {
        let potential_muls = x.split("mul");
        potential_muls.skip(1).for_each(|y| {
            if y.starts_with("(") {
                let close_paren_index = y.find(")").unwrap_or(0);
                if close_paren_index != 0 {
                    let substringed_parens = &y[1..close_paren_index];
                    
                    //println!("{substringed_parens}");
                    let mut nums = substringed_parens.split(",");
                    if nums.clone().count() == 2 {
                        let num1_unparsed = nums.nth(0).unwrap_or("");
                        let num2_unparsed = nums.nth(0).unwrap_or("");

                        if num1_unparsed.chars().count() < 3 && num2_unparsed.chars().count() < 3 {
                            //println!("{num1_unparsed}, {num2_unparsed}");
                            let num1 = num1_unparsed.parse::<i32>().unwrap_or(0);
                            let num2 = num2_unparsed.parse::<i32>().unwrap_or(0);

                            //println!("{num1}, {num2}");
                            output += num1 * num2;
                        }
                    }
                }
            }
        });
    });

    println!("Output: {output}");
}

/*
What makes a valid multiplication request?

mul(X,Y)

- starts with 'mul'
- open paren
- integer max 3 characters
- comma
- integer max 3 characters
- closing paren
*/
