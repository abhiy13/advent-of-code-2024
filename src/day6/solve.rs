use std::{collections::HashSet, result};

use crate::reader::reader;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("WTF"), // Return None if the character doesn't match any direction
        }
    }

    fn next_direction(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_move(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn get_str(&self) -> &str {
        match self {
            Direction::Up => "U",
            Direction::Right => "R",
            Direction::Down => "D",
            Direction::Left => "L",
        }
    }
}

fn get_start(matrix: &Vec<Vec<char>>) -> ((i32, i32), Direction) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '.' && matrix[i][j] != '#' {
                return ((i as i32, j as i32), Direction::from_char(matrix[i][j]));
            }
        }
    }
    panic!("NO start found");
}

const MAX_ITER_TIMES: i32 = 100_00;

fn get_visits(matrix: &Vec<Vec<char>>, start: ((i32, i32), Direction)) -> (i32, bool) {
    let ((mut currx, mut curry), mut direction) = start;
    let mut visisted = HashSet::<(i32, i32)>::new();
    let mut detect_loop = HashSet::<(i32, i32, String)>::new();
    let mut num_times = 0;

    loop {
        if detect_loop.contains(&(currx, curry, direction.clone().get_str().to_string())) {
            return (-1, true);
        }
        // println!("{}", num_times);
        detect_loop.insert((currx, curry, direction.clone().get_str().to_string()));
        visisted.insert((currx, curry));
        let (nx, ny) = direction.get_move();
        currx += nx;
        curry += ny;
        if !(currx >= 0
            && currx < matrix.len() as i32
            && curry >= 0
            && curry < matrix[0].len() as i32)
        {
            break;
        }
        if matrix[currx as usize][curry as usize] == '#' {
            currx -= nx;
            curry -= ny;
            direction = direction.next_direction();
        }
        num_times += 1;
    }
    return (visisted.len() as i32, false);
}

pub fn part1(lines: Vec<String>) {
    let matrix = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!("{}", get_visits(&matrix, get_start(&matrix)).0);
}

pub fn part2(lines: Vec<String>) {
    let mut matrix = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut result = 0;
    let start = get_start(&matrix);
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != '.' {
                continue;
            }
            let dx = matrix[i][j];
            matrix[i][j] = '#';
            if get_visits(&matrix, start.clone()).1 {
                result += 1;
            }
            matrix[i][j] = dx;
        }
    }

    println!("{}", result);
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    // part1(lines);
    part2(lines);

    Ok(())
}
