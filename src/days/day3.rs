use crate::common;
use crate::common::parse_binary;

struct BinaryCounter {
    pub counts: Vec<u64>,
}

impl BinaryCounter {
    fn new(bits: usize) -> BinaryCounter {
        BinaryCounter {
            counts: (0..bits).map(|_| 0).collect(),
        }
    }

    fn consume(&mut self, value: &char, bit: usize) {
        if *value == '1' {
            self.counts[bit] += 1;
        }
    }
}

fn get_count_string(counter: &BinaryCounter, total_lines: usize, use_most_common: bool) -> String {
    let most_common_threshold = (total_lines / 2) as u64;
    let mut result = String::new();
    for one_count in &counter.counts {
        let is_one_most_common = *one_count > most_common_threshold;
        let current_char = if is_one_most_common == use_most_common { '1' } else { '0' };
        result.push(current_char);
    }
    result
}

fn find_power_consumption(lines: &Vec<&str>) -> u128 {
    let mut counter = BinaryCounter::new(lines[0].len());
    for line in lines {
        for (i, value) in line.chars().enumerate() {
            counter.consume(&value, i);
        }
    }
    let total_lines = lines.len();
    let gamma = parse_binary(&get_count_string(&counter, total_lines, true /*use_most_common*/));
    let epsilon = parse_binary(&get_count_string(&counter, total_lines, false /*use_most_common*/));
    gamma * epsilon
}

pub fn run() {
    println!("Day 3");
    let input = common::read("input/day3.txt");
    let input: Vec<&str> = input.lines().collect();
    println!("Part 1: {}", find_power_consumption(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        let input: Vec<&str> = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".trim().lines().collect();

        assert_eq!(198, find_power_consumption(&input));
    }
}