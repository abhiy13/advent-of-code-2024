use crate::reader::reader;
use anyhow::Result;

const MATCH_STRING: [char; 4] = ['X', 'M', 'A', 'S'];

const DX: [i32; 8] = [0, 1, 0, -1, -1, 1, -1, 1];
const DY: [i32; 8] = [1, 0, -1, 0, -1, 1, 1, -1];

pub fn part1(lines: Vec<String>) {
    let matrix = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n: i32 = lines.len().try_into().unwrap();
    let m: i32 = lines[0].len().try_into().unwrap();

    let check_bounds = |i, j| -> bool {
        return i >= 0 && i < n && j >= 0 && j < m;
    };

    matrix[0][0];

    let check_match = |ci, cj, dx, dy| -> bool {
        let mut f = 0;
        let (mut i, mut j) = (ci, cj);
        while f < MATCH_STRING.len() && check_bounds(i, j) {
            if MATCH_STRING[f] != matrix[i as usize][j as usize] {
                break;
            }
            i += dx;
            j += dy;
            f += 1;
        }
        return f == MATCH_STRING.len();
    };

    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            for k in 0..8 {
                result += if check_match(i, j, DX[k], DY[k]) {
                    1
                } else {
                    0
                };
            }
        }
    }

    println!("{}", result);
}

const MATCH_STR: [char; 3] = ['M', 'A', 'S'];

pub fn part2(lines: Vec<String>) {
    let matrix = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = lines.len();
    let m = lines[0].len();

    let mut vm = Vec::from(MATCH_STR);
    vm.sort();

    let check_match = |i: usize, j: usize| -> bool {
        let mut s1 = vec![matrix[i][j], matrix[i - 1][j - 1], matrix[i + 1][j + 1]];
        let mut s2 = vec![matrix[i][j], matrix[i + 1][j - 1], matrix[i - 1][j + 1]];

        s1.sort();
        s2.sort();

        vm == s1 && vm == s2
    };

    let mut result = 0;

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if matrix[i][j] == 'A' {
                result += if check_match(i, j) { 1 } else { 0 };
            }
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
