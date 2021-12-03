use std::fs::File;
use std::io::Read;

fn final_position(horizontal_position: &i32, depth: &i32) -> i32 {
    return horizontal_position * depth;
}

fn parse_input(fname: &str) -> (i32, i32) {
    // get input
    println!("Reading file {}... ", fname);
    let mut file = File::open(fname).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // parse input into results
    let mut horizontal_position = 0;
    let mut depth = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap();

        match direction {
            "forward" => {
                horizontal_position += distance.parse::<i32>().unwrap();
            }
            "up" => {
                depth -= distance.parse::<i32>().unwrap();
            }
            "down" => {
                depth += distance.parse::<i32>().unwrap();
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    return (horizontal_position, depth);
}

fn parse_input_with_aim(fname: &str) -> (i32, i32) {
    // get input
    println!("Reading file {}... ", fname);
    let mut file = File::open(fname).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // parse input into results
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut _aim = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap();

        match direction {
            "forward" => {
                horizontal_position += distance.parse::<i32>().unwrap();
                depth += _aim * distance.parse::<i32>().unwrap();
            }
            "up" => {
                _aim -= distance.parse::<i32>().unwrap();
            }
            "down" => {
                _aim += distance.parse::<i32>().unwrap();
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    return (horizontal_position, depth);
}

fn main() {
    // part 1
    let (horizontal_position, depth) = parse_input("src/test_input.txt");
    println!(
        "Final position: {}",
        final_position(&horizontal_position, &depth)
    );

    let (horizontal_position, depth) = parse_input("src/puzzle_input.txt");
    println!(
        "Final position: {}",
        final_position(&horizontal_position, &depth)
    );

    // part 2
    let (horizontal_position, depth) = parse_input_with_aim("src/test_input.txt");
    println!(
        "Final position: {}",
        final_position(&horizontal_position, &depth)
    );

    let (horizontal_position, depth) = parse_input_with_aim("src/puzzle_input.txt");
    println!(
        "Final position: {}",
        final_position(&horizontal_position, &depth)
    );
}
