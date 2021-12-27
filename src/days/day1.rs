use itertools::Itertools;
use crate::common;
use crate::common::numbers;

pub fn count_increasing_windows_2(values: &[i128]) -> usize {
    values.iter().tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn count_increasing_relative_windows(values: &[i128]) -> usize {
    let window_sums: Vec<i128> = values.windows(3)
        .map(|window| window.iter().sum())
        .collect();

    window_sums.iter().tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn run() {
    println!("Day 1");
    part1();
    part2();
}

fn part1() {
    let input = common::read("input/day1.txt");
    println!("Part 1: {}", count_increasing_windows_2(&numbers(&input)));
}

fn part2() {
    let input = common::read("input/day1.txt");
    println!("Part 2: {}", count_increasing_relative_windows(&numbers(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        let input = numbers("199
200
208
210
200
207
240
269
260
263");

        assert_eq!(7, count_increasing_windows_2(&input));
    }

    #[test]
    pub fn part2() {
        let input = numbers("199
200
208
210
200
207
240
269
260
263");

        assert_eq!(5, count_increasing_relative_windows(&input));
    }
}