use std::{f32::INFINITY, i32};

use crate::reader::reader;
use anyhow::Result;

pub fn part1(lines: Vec<String>) {
    let mut result = 0;

    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let increasing = nums
            .windows(2)
            .all(|w| w[0] < w[1] && (w[1] - w[0] >= 1) && (w[1] - w[0] <= 3));

        let decreasing = nums
            .windows(2)
            .all(|w| w[0] > w[1] && (w[0] - w[1] >= 1) && (w[0] - w[1] <= 3));

        result += if increasing || decreasing { 1 } else { 0 };
    }

    println!("{}", result);
}

fn check_sat<F>(mut nums: Vec<i32>, sat_fn: F) -> bool
where
    F: Fn(&[i32]) -> bool,
{
    for i in 1..nums.len() {
        if !sat_fn(&[nums[i - 1], nums[i]]) {
            nums.remove(i);
            break;
        }
    }

    nums.windows(2).all(|w| sat_fn(w))
}

pub fn part2_brute(lines: Vec<String>) {
    let mut result = 0;

    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let increasing = {
            let mut sat = nums
                .windows(2)
                .all(|w| w[0] < w[1] && (w[1] - w[0] >= 1) && (w[1] - w[0] <= 3));
            for i in 0..nums.len() {
                let mut nms = nums.clone();
                nms.remove(i);
                let increasing = nms
                    .windows(2)
                    .all(|w| w[0] < w[1] && (w[1] - w[0] >= 1) && (w[1] - w[0] <= 3));
                sat |= increasing;
            }
            sat
        };
        let decreasing = {
            let mut sat = nums
                .windows(2)
                .all(|w| w[0] > w[1] && (w[0] - w[1] >= 1) && (w[0] - w[1] <= 3));
            for i in 0..nums.len() {
                let mut nms = nums.clone();
                nms.remove(i);
                let decreasing = nms
                    .windows(2)
                    .all(|w| w[0] > w[1] && (w[0] - w[1] >= 1) && (w[0] - w[1] <= 3));
                sat |= decreasing;
            }
            sat
        };
        result += if increasing || decreasing { 1 } else { 0 };
    }

    println!("{}", result);
}

pub fn part2(lines: Vec<String>) {
    let mut result = 0;

    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let increasing = {
            let mut inc = nums.clone();
            let sat = check_sat(inc, |w: &[i32]| {
                w[0] < w[1] && (w[1] - w[0] >= 1) && (w[1] - w[0] <= 3)
            });
            inc = nums.clone();
            inc.remove(0);
            let increasing = inc
                .windows(2)
                .all(|w| w[0] < w[1] && (w[1] - w[0] >= 1) && (w[1] - w[0] <= 3));
            sat || increasing
        };

        let decreasing = {
            let mut dec = nums.clone();
            let sat = check_sat(dec, |w: &[i32]| {
                w[0] > w[1] && (w[0] - w[1] >= 1) && (w[0] - w[1] <= 3)
            });
            dec = nums.clone();
            dec.remove(0);
            let decreasing = dec
                .windows(2)
                .all(|w| w[0] > w[1] && (w[0] - w[1] >= 1) && (w[0] - w[1] <= 3));
            decreasing || sat
        };

        result += if increasing || decreasing { 1 } else { 0 };
    }

    println!("{}", result);
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    // part1(lines);
    part2_brute(lines);

    Ok(())
}
