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
    let mut _count = 0;

    for i in 3..input.len() {
        let current_start = i - 2;
        let current_end = i + 1;
        let current_triplet: i32 = input[current_start..current_end].iter().sum();

        let last_start = current_start - 1;
        let last_end = current_end - 1;
        let last_triplet: i32 = input[last_start..last_end].iter().sum();

        if current_triplet > last_triplet {
            _count += 1;
        }
    }

    return _count;
}

fn main() {
    // part 1
    let input = get_vector("src/test_input.txt");
    println!("Test Result: {}", count_increases(&input));

    let input = get_vector("src/puzzle_input.txt");
    println!("Test Result: {}", count_increases(&input));

    // part 2
    let input = get_vector("src/test_input.txt");
    println!("Test Result: {}", count_sliding_increases(&input));

    let input = get_vector("src/puzzle_input.txt");
    println!("Test Result: {}", count_sliding_increases(&input));
}
