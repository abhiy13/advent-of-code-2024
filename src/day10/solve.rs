use crate::reader::reader;
use anyhow::Result;

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs_count(i: usize, j: usize, matrix: &Vec<Vec<i32>>, sums: &mut Vec<Vec<i32>>) -> i32 {
    if matrix[i][j] == 9 {
        sums[i][j] = 1;
        return 1;
    }
    let mut sum = 0;
    let mut count = 0;
    for (x, y) in DX.iter().zip(DY) {
        let dx = i as i32 + x;
        let dy = j as i32 + y;
        if dx < 0 || dx >= matrix.len() as i32 || dy < 0 || dy >= matrix[0].len() as i32 {
            continue;
        }
        if matrix[dx as usize][dy as usize] != matrix[i][j] + 1 {
            continue;
        }
        if sums[dx as usize][dy as usize] == -1 {
            count += dfs_count(dx as usize, dy as usize, matrix, sums);
        }
        sum += sums[dx as usize][dy as usize];
    }
    sums[i][j] = sum;
    return count;
}

pub fn solve() -> Result<()> {
    let lines = reader::read();

    let matrix: Vec<Vec<i32>> = lines
        .iter()
        .map(|x| x.chars().map(|x| x as i32 - '0' as i32).collect())
        .collect();
    let n = matrix.len();
    let m = matrix[0].len();

    let mut result = 0;
    let mut result2 = 0;

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 0 {
                let mut sums = vec![vec![-1; m]; n];
                result += dfs_count(i, j, &matrix, &mut sums);
                result2 += sums[i][j];
            }
        }
    }

    println!("Part1: {}", result);
    println!("Part2: {}", result2);

    Ok(())
}
