use crate::reader::reader;
use anyhow::Result;
use regex::Regex;

pub fn part1(lines: Vec<String>) {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for line in lines {
        for cap in re.captures_iter(&line) {
            if let (Ok(num1), Ok(num2)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                sum += num1 * num2;
            }
        }
    }

    println!("{}", sum);
}

pub fn part2(lines: Vec<String>) {
    let mut sum = 0;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut skip = false;
    for line in lines {
        for cap in re.captures_iter(&line) {
            let instruction = cap[0].parse::<String>().unwrap();
            if instruction == "do()" {
                skip = false;
            } else if instruction == "don't()" {
                skip = true;
            } else if let (Ok(num1), Ok(num2)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                if !skip {
                    sum += num1 * num2;
                }
            }
        }
    }

    println!("{}", sum);
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    // part1(lines);
    part2(lines);

    Ok(())
}
