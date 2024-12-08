use std::collections::{HashMap, HashSet};

use crate::reader::reader;
use anyhow::Result;

pub fn part1(lines: Vec<String>) {
    let n = lines.len() as i32;
    let m = lines[0].len() as i32;
    let mut pairs = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut set = HashSet::<(i32, i32)>::new();
    for i in 0..(n as usize) {
        let arr: Vec<char> = lines[i].chars().collect();
        for j in 0..(m as usize) {
            if arr[j] == '.' {
                continue;
            }
            if !pairs.contains_key(&arr[j]) {
                pairs.insert(arr[j], Vec::new());
            }
            pairs.get_mut(&arr[j]).unwrap().push((i as i32, j as i32));
        }
    }

    let check_bounds = |x: i32, y: i32| -> bool { x >= 0 && x < n && y >= 0 && y < m };

    for (_, v) in pairs.into_iter() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let dx = v[i].0 + (v[i].0 - v[j].0);
                let dy = v[i].1 + (v[i].1 - v[j].1);
                if check_bounds(dx, dy) {
                    set.insert((dx, dy));
                }
            }
        }
    }

    println!("{}", set.len());
}

pub fn part2(lines: Vec<String>) {
    let n = lines.len() as i32;
    let m = lines[0].len() as i32;
    let mut pairs = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut set = HashSet::<(i32, i32)>::new();
    for i in 0..(n as usize) {
        let arr: Vec<char> = lines[i].chars().collect();
        for j in 0..(m as usize) {
            if arr[j] == '.' {
                continue;
            }
            if !pairs.contains_key(&arr[j]) {
                pairs.insert(arr[j], Vec::new());
            }
            pairs.get_mut(&arr[j]).unwrap().push((i as i32, j as i32));
        }
    }

    let check_bounds = |x: i32, y: i32| -> bool { x >= 0 && x < n && y >= 0 && y < m };

    for (_, v) in pairs.into_iter() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let mut dx = v[i].0;
                let mut dy = v[i].1;
                while check_bounds(dx, dy) {
                    set.insert((dx, dy));
                    dx += v[i].0 - v[j].0;
                    dy += v[i].1 - v[j].1;
                }
            }
        }
    }

    println!("{}", set.len());
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    // part1(lines);
    part2(lines);

    Ok(())
}
