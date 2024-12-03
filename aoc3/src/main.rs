use std::fs;

use regex::Regex;


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let total: i64 = reg.captures_iter(&input)
        .map(|x| {
            let (_, groups): (&str, [&str; 2]) = x.extract();
            let nums: i64 = groups.iter().filter_map(|x| x.parse::<i64>().ok()).product();
            nums
        }).sum();
    println!("{:?}", total);
}
