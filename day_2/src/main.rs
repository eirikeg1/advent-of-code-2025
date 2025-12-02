use std::{
    fs::{self},
    io::{BufRead, Read},
};

fn main() {
    let input_1_file = "src/inputs/1.txt";

    let content = fs::read_to_string(input_1_file).expect("Error reading input file");

    let ranges = content.split(",");

    let mut invalid_task_1 = Vec::<i64>::new();
    let mut invalid_task_2 = Vec::<i64>::new();

    for range in ranges.into_iter() {
        let splits: Vec<&str> = range.split("-").collect();

        let (start_range, end_range) = (
            splits[0].parse::<i64>().unwrap(),
            splits[1].parse::<i64>().unwrap(),
        );

        for i in start_range..=end_range {
            let i_string = i.to_string();

            // Task 1
            let (left_split, right_split) = i_string.split_at(i_string.len() / 2);
            if left_split == right_split {
                invalid_task_1.push(i);
            }

            // Task 2
            let str_len = i_string.len();
            let mut substrings = Vec::<&str>::new();
            for j in 0..str_len {
                for k in j + 1..str_len {
                    substrings.push(&i_string[j..k]);
                }
            }
            if substrings.iter().any(|s| check_if_invalid(&i_string, s)) {
                invalid_task_2.push(i);
            }
        }
    }

    let task_1_sum: i64 = invalid_task_1.into_iter().sum();
    let task_2_sum: i64 = invalid_task_2.into_iter().sum();

    println!("The number of invalid for task 1 is {}", task_1_sum);
    println!("The number of invalid for task 2 is {task_2_sum}");
}

fn check_if_invalid(full_str: &str, sub_str: &str) -> bool {
    full_str.to_string() == sub_str.repeat(full_str.len() / sub_str.len())
}
