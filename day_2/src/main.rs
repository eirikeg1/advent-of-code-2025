use std::{fs::{self}, io::{BufRead, Read}};

fn main() {
    let input_1_file = "src/inputs/1.txt";

    let content = fs::read_to_string(input_1_file).expect("Error reading input file");

    let ranges = content.split(",");
    
    let mut invalid = Vec::<i64>::new();


    for range in ranges.into_iter() {

        let splits: Vec<&str> = range.split("-").collect();

        println!("{splits:?}");
        let (start_range, end_range) = (
            splits[0].parse::<i64>().unwrap(),
            splits[1].parse::<i64>().unwrap()
        );

        for i in start_range..=end_range {

            let i_string = i.to_string();

            let (left_split, right_split) = i_string.split_at(i_string.len() / 2);
            if left_split == right_split {
                invalid.push(i);
            }
        }        
    }

    let sum: i64 = invalid
        .into_iter()
        .sum();

    println!("The sum is {sum}");
}
