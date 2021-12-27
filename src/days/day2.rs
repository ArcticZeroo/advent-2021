use crate::turtle::Turtle3D;
use crate::{common, world};

struct MoveCommand {
    direction: world::MovementDirection,
    count: i64,
}

fn parse_line(line: &str) -> MoveCommand {
    let pieces = line.split_once(" ")
        .expect("Command could not be split into pieces");

    let direction = match pieces.0 {
        "forward" => world::MovementDirection::Forward,
        "down" => world::MovementDirection::Down,
        "up" => world::MovementDirection::Up,
        other => panic!("Unexpected direction '{:?}'", other)
    };

    let count: i64 = pieces.1.parse().expect("Could not parse count.");

    MoveCommand {
        direction,
        count,
    }
}

fn parse(input: &str) -> Vec<MoveCommand> {
    input.trim()
        .lines()
        .map(parse_line)
        .collect()
}

fn find_position_value(commands: Vec<MoveCommand>) -> i64 {
    let mut turtle = Turtle3D::new();
    for command in commands {
        turtle.do_move(command.direction, command.count);
    }
    (turtle.position.z * turtle.position.x).abs()
}

fn find_aim_value(commands: Vec<MoveCommand>) -> i64 {
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;

    for command in commands {
        match command.direction {
            world::MovementDirection::Down => aim += command.count,
            world::MovementDirection::Up => aim -= command.count,
            world::MovementDirection::Forward => {
                horizontal += command.count;
                depth += aim * command.count;
            }
            _ => panic!("Invalid direction {:?}", command.direction)
        }
    }

    (horizontal * depth).abs()
}

pub fn run() {
    println!("Day 2");
    part1();
    part2();
}

fn part1() {
    let commands = parse(&common::read("input/day2.txt"));
    println!("Part 1: {}", find_position_value(commands));
}

fn part2() {
    let commands = parse(&common::read("input/day2.txt"));
    println!("Part 2: {}", find_aim_value(commands));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        let commands = parse("forward 5
down 5
forward 8
up 3
down 8
forward 2");

        assert_eq!(150, find_position_value(commands));
    }

    #[test]
    pub fn part2() {
        let commands = parse("forward 5
down 5
forward 8
up 3
down 8
forward 2");

        assert_eq!(900, find_aim_value(commands));
    }
}