use std::collections::VecDeque;

use crate::reader::reader;
use anyhow::Result;

fn part1(lines: Vec<String>) {
    let input: Vec<char> = lines[0].chars().collect();
    let mut total_len = 0;

    let mut dequeue = VecDeque::<(i64, u64)>::new();

    for i in 0..input.len() {
        let num = input[i] as u64 - '0' as u64;
        if i % 2 == 0 {
            dequeue.push_back(((i as i64) / 2, num));
        } else {
            dequeue.push_back((-1, num));
        }
        total_len += num;
    }

    let mut sum: u64 = 0;

    for i in 0..total_len {
        while !dequeue.is_empty() && dequeue.front().unwrap().1 == 0 {
            dequeue.pop_front();
        }

        if dequeue.is_empty() {
            break;
        }

        let top = dequeue.front_mut().unwrap();
        top.1 -= 1;

        let num_here = if top.0 == -1 {
            while !dequeue.is_empty()
                && (dequeue.back().unwrap().0 == -1 || dequeue.back().unwrap().1 == 0)
            {
                dequeue.pop_back();
            }
            if dequeue.is_empty() {
                0
            } else {
                let back = dequeue.back_mut().unwrap();
                back.1 -= 1;
                back.0
            }
        } else {
            top.0
        };

        sum += i * (num_here as u64);
    }

    println!("{}", sum);
}

#[derive(Debug)]
struct Container {
    index: usize,
    count: usize,
    value: usize,
}

fn part2(lines: Vec<String>) {
    let input: Vec<char> = lines[0].chars().collect();

    let mut spaces = Vec::<Container>::new();
    let mut numbers = Vec::<Container>::new();

    let mut curr_index = 0;

    for i in 0..input.len() {
        let num = input[i] as usize - '0' as usize;
        if i % 2 == 0 {
            numbers.push(Container {
                index: curr_index,
                count: num,
                value: i / 2,
            });
        } else {
            spaces.push(Container {
                index: curr_index,
                count: num,
                value: i / 2,
            });
        }
        curr_index += num;
    }

    numbers.reverse();
    for i in 0..numbers.len() {
        for j in 0..spaces.len() {
            if spaces[j].count < numbers[i].count {
                continue;
            }
            if spaces[j].index > numbers[i].index {
                continue;
            }
            let index = spaces[j].index;
            spaces[j].count -= numbers[i].count;
            spaces[j].index += numbers[i].count;
            numbers[i].index = index;
        }
    }

    let mut area = vec![0; curr_index as usize];
    let mut sum: i64 = 0;

    for x in numbers {
        for i in x.index..x.index + x.count {
            area[i] = x.value;
        }
    }

    for i in 0..curr_index {
        sum += i as i64 * area[i] as i64;
    }

    println!("{}", sum);
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    // part1(lines);
    part2(lines);

    Ok(())
}
