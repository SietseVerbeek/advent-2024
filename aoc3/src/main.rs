use std::fs;

use regex::Regex;


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let total1: i64 = reg.captures_iter(&input)
        .map(|x| {
            let (_, groups): (&str, [&str; 2]) = x.extract();
            let nums: i64 = groups.iter().filter_map(|x| x.parse::<i64>().ok()).product();
            nums
        }).sum();

    let strings: Vec<&str> = input.split("do()")
        .filter_map(|x| x.split("don't").next())
        .collect();

    let total2: i64 = strings.into_iter()
        .map(|x| {
            reg.captures_iter(&x)
                .map(|x| {
                    let (_, groups): (&str, [&str; 2]) = x.extract();
                    let nums: i64 = groups.iter().filter_map(|x| x.parse::<i64>().ok()).product();
                    nums
                }).sum::<i64>()
        }).sum();

    println!("{:?}", total1);
    println!("{:?}", total2);
}
