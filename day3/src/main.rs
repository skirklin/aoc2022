#![allow(unused_imports)]
#![allow(dead_code)]

use utils;
use regex::Regex;
use std::collections::HashSet;

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        1 + (c as i32) - ('a' as i32)
    } else {
        27 + (c as i32) - ('A' as i32)
    }
}

fn part1(input: String) {
    let mut prio = 0;
    for line in input.trim_end().split("\n") {
        let (first, second) = line.split_at(line.len()/2);
        let mut set: HashSet<char> = HashSet::new();
        for c in first.chars() {
            set.insert(c);
        }
        let mut dup: Option<char> = None;
        for c in second.chars() {
            if set.contains(&c) {
                dup = Some(c)
            }
        }
        prio += priority(dup.unwrap())
    }
    println!("Total priority: {}", prio)
}

fn part2(input: String) {
    let mut prio = 0;
    let mut lines = input.trim_end().split("\n").peekable();
    while lines.peek().is_some() {
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        let chars_a: Vec<i32> = a.chars().map(priority).collect();
        let chars_b: Vec<i32> = b.chars().map(priority).collect();
        let chars_c: Vec<i32> = c.chars().map(priority).collect();
        let ans: Vec<&i32> = chars_a.iter()
          .filter(|p| { chars_b.contains(p) })
          .filter(|p| { chars_c.contains(p) })
          .collect();
        prio += ans[0];
    };

    println!("Total priority: {}", prio)
}

const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";


fn main() {
    let input = utils::fetch_input(3, 2022);
    // let input = String::from(TEST_INPUT);
    // part1(input);
    part2(input);
}
