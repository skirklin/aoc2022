#![allow(unused_imports)]
#![allow(dead_code)]


use utils;

fn part1(input: String) {
    let mut elves: Vec<u32> = vec![];
    let mut curr_elf: u32 = 0;
    for line in input.split("\n") {
        if line == "" {
            elves.push(curr_elf);
            curr_elf = 0;
            continue
        }
        let num = line.parse::<u32>().unwrap();
        curr_elf += num
    }

    elves.sort_unstable();
    print!("\nthe elf with the most calories is carrying {} calories\n\n", elves[elves.len() - 1])
}

fn part2(input: String) {
    let mut elves: Vec<u32> = vec![];
    let mut curr_elf: u32 = 0;
    for line in input.split("\n") {
        if line == "" {
            elves.push(curr_elf);
            curr_elf = 0;
            continue
        }
        let num = line.parse::<u32>().unwrap();
        curr_elf += num
    }
    elves.push(curr_elf);

    elves.sort_unstable();
    print!("\nthe 3 elves with the most calories are carrying {} calories\n\n", elves[elves.len() - 3] + elves[elves.len() - 2] + elves[elves.len() - 1])
}

const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";


fn main() {
    let input = utils::fetch_input(1, 2022);
    // part2(String::from(TEST_INPUT))
    part2(String::from(input))
}
