use std::{io::{self, BufRead}, fs::File};

fn main() -> io::Result<()>{
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);

    let lines: Vec<Vec<i16>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i16>().ok())
                .collect()
        }).collect();

    let count1 = lines.iter()
        .map(|line| {
            line.windows(2).map(|win| {
                let [x, y] = win else { panic!() };
                x - y
            }).collect::<Vec<i16>>()
        }).filter(|x| { 
            (
                1 <= *x.iter().min().unwrap() && 
            *x.iter().max().unwrap() <= 3
        ) || 
            (-3 <= *x.iter().min().unwrap() && 
            *x.iter().max().unwrap() <= -1)
        }).count();

    let count2 = lines.iter()
        .map(|line| {
            line.windows(2).map(|win| {
                let [x, y] = win else { panic!() };
                x - y
            }).collect::<Vec<i16>>()
        }).map(|line| {
            line.windows(2).map(|win| {
                let [x, y] = win else { panic!() };
                let sum = x + y;
                sum
            }).collect::<Vec<i16>>()
        });

    println!("Part 2");
    for l in count2 {
        println!("{:?}", l);
    };

    println!("Part 1");
    println!("{:?}", count1);
    Ok(())
}
