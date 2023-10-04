use std::env;
use std::fs;
use std::io::Write;

use reqwest::header::COOKIE;

const INPUT_CACHE_ROOT: &str = "/tmp/aoc";
type Year = i16;
type Day = i16;

fn fetch_input_session(day: Day, year: Year, session_id: String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    print!("fetching inputs from {}\n", url);
    let res = client
        .get(url)
        .header(COOKIE, format!("session={}", session_id))
        .send();
    return res?.text();
}

fn cache_path(day: Day, year: Year) -> String {
    format!("{}/{}/{}", INPUT_CACHE_ROOT, year, day)
}

fn cache_file(day: Day, year: Year) -> String {
    format!("{}/input.txt", cache_path(day, year))
}

fn read_cached_input(day: Day, year: Year) -> Option<String> {
    let path = cache_path(day, year);
    match fs::read_dir(path.as_str()) {
        Err(_) => {
            print!("cache miss: {}\n", path);
            None
        },
        Ok(_) => {
            print!("cache hit: {}\n", path);
            fs::read_to_string(cache_file(day, year)).ok()
        }
    }
}

fn cache_input(day: Day, year: Year, input: &String) {
    let path = cache_path(day, year);
    fs::create_dir_all(path).unwrap();
    let _ = fs::remove_file(cache_file(day, year));
    let f: Result<fs::File, std::io::Error> = fs::File::create(cache_file(day, year));
    f.unwrap().write_all(input.as_bytes()).unwrap();
}

pub fn fetch_input(day: i16, year: i16) -> String {
    let session = env::var("AOC_SESSION_ID").unwrap();

    match read_cached_input(day, year) {
        None => {
            let input = fetch_input_session(day, year, session).unwrap();
            cache_input(day, year, &input);
            input
        },
        Some(input) => {
            input
        }
    }
}