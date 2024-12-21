use std::collections::HashSet;

use crate::reader::reader;
use anyhow::Result;

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs_perimeter(
    x: i32,
    y: i32,
    visited: &mut HashSet<(i32, i32)>,
    matrix: &Vec<Vec<char>>,
) -> i32 {
    visited.insert((x, y));
    let mut perimeter = 0;
    for (dx, dy) in DX.iter().zip(DY) {
        let ex = x + *dx;
        let ey = y + dy;
        if ex < 0 || ex >= matrix.len() as i32 || ey < 0 || ey >= matrix[0].len() as i32 {
            perimeter += 1;
        } else if matrix[ex as usize][ey as usize] != matrix[x as usize][y as usize] {
            perimeter += 1;
        } else {
            if visited.contains(&(ex, ey)) {
                continue;
            }
            perimeter += dfs_perimeter(ex, ey, visited, matrix);
        }
    }
    return perimeter;
}

pub fn solve() -> Result<()> {
    let lines = reader::read();

    let matrix: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    let n = matrix.len();
    let m = matrix[0].len();

    let mut visited = HashSet::<(i32, i32)>::new();

    let mut result = 0;

    for i in 0..n as i32 {
        for j in 0..m as i32 {
            if visited.contains(&(i, j)) {
                continue;
            }
            let curr = visited.len() as i32;
            let perimeter = dfs_perimeter(i, j, &mut visited, &matrix);
            result += perimeter * (visited.len() as i32 - curr);
        }
    }

    println!("{}", result);
    Ok(())
}
