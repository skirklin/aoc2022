#![allow(unused_imports)]
#![allow(dead_code)]

use utils;
use regex::Regex;

#[derive(Copy, Clone)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

fn parse_line(line: String) -> i32 {
    let pattern = Regex::new(r"(?<them>[ABC]) (?<me>[XYZ])$").unwrap();
    let caps = pattern.captures(&line).unwrap();
    let them = match &caps["them"] {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        &_ => panic!("unhandled case")
    };

    let me = match &caps["me"] {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        &_ => panic!("unhandled case")
    };

    let (my_score, _) = score(me as i32, them as i32);

    return my_score;
}

fn score(a: i32, b: i32) -> (i32, i32) {
    let diff = (3 + b - a) % 3;
    let a_score = match diff {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => panic!("can't happen")
    };
    return (a_score + a + 1, 6 - a_score + b + 1)
}

fn part1(input: String) {
    let mut score = 0;
    for line in input.trim_end_matches("\n").split("\n") {
        let line_score = parse_line(String::from(line));
        println!("line: {} -> {}", line, line_score);
        score += line_score
    }
    println!("Total score for game was: {}", score)
}

fn parse_line_2(line: String) -> i32 {
    let pattern = Regex::new(r"(?<them>[ABC]) (?<me>[XYZ])$").unwrap();
    let caps = pattern.captures(&line).unwrap();
    let them = match &caps["them"] {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        &_ => panic!("unhandled case")
    } as i32;

    let me = match &caps["me"] {
        "X" => {
            // need to lose
            (them - 1 + 3 ) % 3
        },
        "Y" => {
            // need to draw
            them
        },
        "Z" => {
            // need to win
            (them + 1) % 3
        },
        &_ => panic!("unhandled case")
    };

    let (my_score, _) = score(me, them);

    return my_score;
}

fn part2(input: String) {
    let mut score = 0;
    for line in input.trim_end_matches("\n").split("\n") {
        let line_score = parse_line_2(String::from(line));
        println!("line: {} -> {}", line, line_score);
        score += line_score
    }
    println!("Total score for game was: {}", score)
}

const TEST_INPUT: &str = "A Y
B X
C Z";


fn main() {
    let input = utils::fetch_input(2, 2022);
    // let input = String::from(TEST_INPUT);
    // part1(input);
    part2(input);
}
