use std::fs;

pub fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Could not read input")
}

pub fn numbers(input: &str) -> Vec<i128> {
    input.trim()
        .lines()
        .map(|line| line
            .parse()
            .expect("Invalid number"))
        .collect()
}

pub fn parse_binary(value: &str) -> u128 {
    u128::from_str_radix(value, 2).expect("Could not parse binary")
}