use crate::reader::reader;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Operations {
    Add,
    Mul,
    Concat,
}

impl Operations {
    fn safe_mul(a: i64, b: i64) -> i64 {
        match a.checked_mul(b) {
            Some(x) => x,
            None => i64::MAX,
        }
    }

    fn get_concat(a: i64, b: i64) -> i64 {
        let sp = a.to_string() + &b.to_string();
        match sp.parse::<i64>() {
            Ok(x) => x,
            Err(_) => i64::MAX,
        }
    }

    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operations::Add => a + b,
            Operations::Mul => Self::safe_mul(a, b),
            Operations::Concat => Self::get_concat(a, b),
        }
    }
}

fn can_solve(
    required_result: i64,
    array: &Vec<i64>,
    index: usize,
    running_result: i64,
    operations: &Vec<Operations>,
) -> bool {
    if index == array.len() {
        return required_result == running_result;
    }
    if running_result > required_result {
        return false;
    }
    let mut can_sat = false;
    for op in operations {
        can_sat |= can_solve(
            required_result,
            array,
            index + 1,
            op.apply(running_result, array[index]),
            operations,
        );
    }
    can_sat
}

pub fn solve() -> Result<()> {
    let operations_part1 = vec![Operations::Add, Operations::Mul];
    let operations_part2 = vec![Operations::Add, Operations::Mul, Operations::Concat];

    let lines = reader::read();
    let mut result1 = 0;
    let mut result2 = 0;
    for line in lines {
        let mut nums = line
            .split(" ")
            .map(|x| {
                if x.contains(':') {
                    x.strip_suffix(":").unwrap()
                } else {
                    x
                }
            })
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let required_result = nums[0];
        nums.remove(0);

        // part1
        if can_solve(required_result, &nums, 0, 0, &operations_part1) {
            result1 += required_result;
        }

        // part2
        if can_solve(required_result, &nums, 0, 0, &operations_part2) {
            result2 += required_result;
        }
    }

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
