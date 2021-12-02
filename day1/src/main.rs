// https://adventofcode.com/2021/day/1

use std::fs::File;
use std::io::Read;

fn get_vector(fname: &str) -> Vec<i32> {
    // get string from file
    println!("Reading file {}... ", fname);
    let mut file = File::open(fname).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // convert to vector of ints
    let mut vec = Vec::new();
    for line in contents.lines() {
        vec.push(line.parse::<i32>().unwrap());
    }

    return vec;
}

fn count_increases(input: &[i32]) -> i32 {
    let mut last_value = input[0];
    let mut _count = 0;
    for &value in input {
        if value > last_value {
            _count += 1;
        }
        last_value = value;
    }

    return _count;
}

fn count_sliding_increases(input: &[i32]) -> i32 {
    let mut last_value = input[0];
    let mut _count = 0;
    for &value in input {
        if value > last_value {
            _count += 1;
        }
        last_value = value;
    }

    return _count;
}

fn main() {
    let test_input = get_vector("src/test_input_part1.txt");
    println!("Test Input: {:?}", test_input);
    println!("Test Result: {}", count_increases(&test_input));

    let real_input = get_vector("src/puzzle_input_part1.txt");
    println!("Test Input: {:?}", real_input);
    println!("Test Result: {}", count_increases(&real_input));
}
