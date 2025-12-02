use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
enum Rotation {
    Left(i32),
    Right(i32),
}

#[derive(Clone)]
struct CurrentPosition {
    position: i32,
    times_at_zero: i32,
}

fn main() {
    let mut rotation_points = Vec::<i32>::new();
    let mut total_times_at_zero = 0;

    let input_1_file = "src/inputs/input-1.txt";
    let file = File::open(input_1_file).expect("Could not open file");
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.expect("failed to read line");

        let direction = line.chars().next().expect("Empty line");
        let number = &line[1..]
            .parse::<i32>()
            .expect("Second part of line not a number");
        let rotation = match direction {
            'L' => Some(Rotation::Left(*number)),
            'R' => Some(Rotation::Right(*number)),
            _ => None,
        }
        .expect("Error in direction parsing");

        let current_position = *rotation_points.last().unwrap_or(&50);

        let CurrentPosition {
            position: new_position,
            times_at_zero,
        } = calculate_position(rotation, current_position);
        rotation_points.push(new_position);

        total_times_at_zero += times_at_zero;
        
        // println!("The dial is rotated {} to point at {}", line, new_position);
    }

    let number_of_0_values = rotation_points.iter().filter(|&&x| x == 0).count();

    println!(
        "The dial pointed at 0 a total of {} times. Total times at zero counted during wrapping: {}",
        number_of_0_values, total_times_at_zero
    );
}

fn calculate_position(rotation: Rotation, current_position: i32) -> CurrentPosition {
    let mut times_at_zero = 0;
    let mut position = current_position;
    
    // 1. Determine direction and distance
    let (step, amount) = match rotation {
        Rotation::Left(n) => (-1, n),
        Rotation::Right(n) => (1, n),
    };

    // 2. Simulate clicks. This handles the "Start at 0" edge case perfectly.
    for _ in 0..amount {
        position += step;

        // Wrap the dial manually
        if position < 0 { position = 99; }
        if position > 99 { position = 0; }

        // Count the click
        if position == 0 {
            times_at_zero += 1;
        }
    }

    CurrentPosition {
        position,
        times_at_zero,
    }
}


