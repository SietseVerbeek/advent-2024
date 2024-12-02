use std::{fs::{self, File}, io::{self, BufRead}, iter::zip, collections::HashMap};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = io::BufReader::new(file);

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let mut nums = line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok());
            nums.next().zip(nums.next())
        })
    .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let total: i32 = zip(left.clone(), right.clone()).map(|(x, y)| {
        (x - y).abs()
    }).sum();

    println!("Part 1:");
    println!("{:?}", total);

    let hash = right.into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut map, x| {
        *map.entry(x).or_default() += 1;
        map
    });

    let total2 = left.into_iter()
        .fold(0, |acc, x| {
            acc + x * hash.get(&x).unwrap_or(&0)
        });

    println!("Part 2:");
    println!("{:?}", total2);

    Ok(())
}
