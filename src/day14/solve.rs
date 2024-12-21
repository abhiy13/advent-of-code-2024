use std::{i32, result};

use crate::reader::reader;
use anyhow::Result;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn parse_input(lines: Vec<String>) -> Vec<Robot> {
    let mut response = Vec::new();

    let parse_coods = |s: &str| -> (i32, i32) {
        let split: Vec<&str> = s.split(",").collect();
        let x = split[0][2..].parse::<i32>().unwrap();
        let y = split[1].parse::<i32>().unwrap();
        (x, y)
    };

    for i in 0..lines.len() {
        let split: Vec<&str> = lines[i].split_whitespace().collect();
        let (x, y) = parse_coods(split[0]);
        let (vx, vy) = parse_coods(split[1]);
        response.push(Robot { x, y, vx, vy });
    }

    response
}

fn solve_one(robots: &Vec<Robot>, n: i32, m: i32, num_steps: i32) -> (i32, i32) {
    let mut f = [[0, 0], [0, 0]];
    let mut result = 0;

    let mut xy = Vec::<(i32, i32)>::new();
    let mut abs_dist = 0;

    for robot in robots {
        let final_pos_x = ((robot.vx * num_steps) % n + robot.x + n) % n;
        let final_pos_y = ((robot.vy * num_steps) % m + robot.y + m) % m;

        for (fx, fy) in xy.clone().into_iter() {
            abs_dist += (fx - final_pos_x).abs() + (fy - final_pos_y).abs();
        }

        xy.push((final_pos_x, final_pos_y));

        let (mut x, mut y) = (-1, -1);
        if final_pos_x < n / 2 {
            x = 0;
        } else if final_pos_x > n / 2 {
            x = 1;
        }
        if final_pos_y < m / 2 {
            y = 0;
        } else if final_pos_y > m / 2 {
            y = 1;
        }
        if x == -1 || y == -1 {
            continue;
        }
        f[x as usize][y as usize] += 1;
    }

    for i in 0..2 {
        for j in 0..2 {
            result *= f[i][j];
        }
    }

    return (result, abs_dist);
}

pub fn solve() -> Result<()> {
    let lines = reader::read();

    let robots = parse_input(lines);
    let mut result = 1;
    let mut result2 = 0;

    // println!("{:?}", games);

    let num_steps = 100;
    let n = 101;
    let m = 103;

    let (mut minx, mut minv) = (i32::MAX, i32::MAX);

    // find the point with lowest density
    for i in 0..20000 {
        let res = solve_one(&robots, n, m, i);
        if res.1 < minv {
            minv = res.1;
            minx = i;
        }
    }

    println!("Part1: {}", solve_one(&robots, n, m, num_steps).0);
    println!("Part2: {}", minx);
    Ok(())
}
