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

fn get_oxygen_generator_rating(matrix: Array2<usize>) {}

fn calculate_power_consumption(input: &str) {
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
    let oxygen_generator_rating = 0;
    let co2_scrubber_rating = 0;
    let life_support_rating = co2_scrubber_rating * oxygen_generator_rating;
    println!("Life Support Rating: {}", life_support_rating);
}

fn main() {
    println!("Test Input...");
    calculate_power_consumption("src/test_input.txt");

    println!("Puzzle Input...");
    calculate_power_consumption("src/puzzle_input.txt");
}
