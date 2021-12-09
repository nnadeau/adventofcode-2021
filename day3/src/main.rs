// https://adventofcode.com/2021/day/3
use ndarray::{Array, Array2, Axis};
use std::fs::File;
use std::io::Read;

fn get_common_elements(matrix: &Array2<usize>) -> Vec<usize> {
    // gamma is most common element of each column
    let mut common_elements: Vec<usize> = vec![];

    // all elements are binary, so if sum of column > nrows/2, most common is 1
    let threshold = matrix.nrows() / 2;
    for col in matrix.axis_iter(Axis(1)) {
        if col.sum() > threshold {
            common_elements.push(1);
        } else {
            common_elements.push(0);
        }
    }

    return common_elements;
}

fn parse_input(fname: &str) -> String {
    println!("Reading file {}... ", fname);

    let mut file = File::open(fname).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return contents;
}

fn contents_to_matrix(contents: &String) -> Array2<usize> {
    let mut bits = vec![];
    let nrows = contents.lines().count();
    let nbits = contents.lines().last().unwrap().len();

    for line in contents.lines() {
        let mut current_bits: Vec<usize> = line
            .chars()
            .map(|x| usize::from_str_radix(&x.to_string(), 2).unwrap())
            .collect();
        bits.append(&mut current_bits);
    }

    let matrix = Array::from_shape_vec((nrows, nbits), bits).unwrap();
    return matrix;
}

fn get_gamma_binary(matrix: &Array2<usize>) -> String {
    let common_elements = get_common_elements(&matrix);

    let gamma_binary: String = common_elements
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("Gamma Binary: {}", gamma_binary);

    return gamma_binary;
}

fn get_epsilon_rate(gamma_rate: &usize, nbits: &usize) -> usize {
    let binary_mask = !(usize::MAX << nbits);
    let epsilon_rate = !gamma_rate & binary_mask;
    println!("Epsilon Binary: {:b}", epsilon_rate);
    println!("Epsilon Rate: {}", epsilon_rate);

    return epsilon_rate;
}

fn get_life_support_rating_metrics(
    numbers: &Vec<usize>,
    nbits: usize,
    is_most_common: bool,
) -> usize {
    let mut current_numbers: Vec<usize> = numbers.clone();
    for i in (0..nbits).rev() {
        // println!("{}", "~".repeat(10));
        // println!("Iteration {}", i);
        // for g in &current_numbers {
        //     println!("{:#07b}\t{}", g, g);
        // }

        // find threshold of "most common"
        let count_threshold = (current_numbers.len() + 1) / 2;
        // println!("Count : {}", current_numbers.len());
        // println!("Threshold: {}", count_threshold);

        // find count of 1 in column
        let ones_count = current_numbers
            .iter()
            .filter(|x| check_bit_for_one(x, i))
            .count();

        // check if 1 is most common
        // println!("ones_count: {}", ones_count);
        let is_ones_common = ones_count >= count_threshold;

        // println!("is_ones_common: {}", is_ones_common);

        // split current_numbers into two groups, keeping common
        let (new_numbers_true, new_numbers_false): (Vec<usize>, Vec<usize>) = current_numbers
            .iter()
            .partition(|x| check_bit_for_one(x, i) == is_ones_common);

        // replace current_numbers
        if is_most_common {
            current_numbers = new_numbers_true;
        } else {
            current_numbers = new_numbers_false;
        }

        // break if one left
        if current_numbers.len() <= 1 {
            break;
        }
    }
    return *current_numbers.last().unwrap();
}

fn check_bit_for_one(number: &usize, bit_idx: usize) -> bool {
    let mask = 1 << bit_idx;
    let masked_value = *number & mask;
    let is_bit_valid = masked_value > 0;

    return is_bit_valid;
}

fn calculate_puzzle(input: &str) {
    let contents = parse_input(input);
    let matrix = contents_to_matrix(&contents);

    // part 1
    let gamma_binary = get_gamma_binary(&matrix);
    let gamma_rate = usize::from_str_radix(&gamma_binary, 2).unwrap();
    println!("Gamma Rate: {}", gamma_rate);

    let epsilon_rate = get_epsilon_rate(&gamma_rate, &gamma_binary.len());
    let power_consumption = gamma_rate * epsilon_rate;
    println!("Power Consumption: {}", power_consumption);

    // part 2
    let nbits = contents.lines().last().unwrap().len();
    let numbers = contents
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect::<Vec<_>>();

    let oxygen_rating = get_life_support_rating_metrics(&numbers, nbits, true);
    println!("Oxygen Generator Rating: {}", oxygen_rating);

    let co2_scrubber_rating = get_life_support_rating_metrics(&numbers, nbits, false);
    println!("CO2 Scrubber Rating: {}", co2_scrubber_rating);

    let life_support_rating = co2_scrubber_rating * oxygen_rating;
    println!("Life Support Rating: {}", life_support_rating);
}

fn main() {
    println!("Test Input...");
    calculate_puzzle("src/test_input.txt");

    println!("{}", "~".repeat(50));

    println!("Puzzle Input...");
    calculate_puzzle("src/puzzle_input.txt");
}
