use std::collections::HashMap;

use anyhow::Result;
use crate::reader::reader;


pub fn part1(lines: Vec<String>) {
    let (mut list_a, mut list_b) = (Vec::new(), Vec::new()); 
    for line in lines {
        let nums: [i32; 2] = line.split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Expected exactly two numbers");

        list_a.push(nums[0]);
        list_b.push(nums[1]);
    }

    list_a.sort();
    list_b.sort();

    let mut sum = 0;

    for (a, b) in list_a.iter().zip(list_b.iter()) {
        sum += (a - b).abs();
    }

    println!("{}", sum);
}


pub fn part2(lines: Vec<String>) {
    let (mut list, mut map) = (Vec::new(), HashMap::new()); 
    for line in lines {
        let nums: [i32; 2] = line.split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Expected exactly two numbers");

        list.push(nums[0]);
        let count = map.get(&nums[1]).unwrap_or(&0);
        map.insert(nums[1], count + 1);
    }

    let mut result = 0;

    for num in list {
        result += num * map.get(&num).unwrap_or(&0);        
        // println!("{} {}", num, map.get(&num).unwrap_or(&0));
    }

    println!("{}", result);
}

pub fn solve() -> Result<()> {

    let lines = reader::read();
    // part1(lines);
    part2(lines);

    Ok(())
}