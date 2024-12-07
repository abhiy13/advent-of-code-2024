use crate::reader::reader;
use anyhow::Result;

fn safe_mul(a: i64, b: i64) -> i64 {
    match a.checked_mul(b) {
        Some(x) => x,
        None => i64::MAX,
    }
}

fn can_solve(required_result: i64, array: &Vec<i64>, index: usize, running_result: i64) -> bool {
    if index == array.len() {
        return required_result == running_result;
    }
    if running_result > required_result {
        return false;
    }
    return can_solve(
        required_result,
        array,
        index + 1,
        running_result + array[index],
    ) || can_solve(
        required_result,
        array,
        index + 1,
        safe_mul(running_result, array[index]),
    );
}

fn get_concat(a: i64, b: i64) -> i64 {
    let sp = a.to_string() + &b.to_string();
    match sp.parse::<i64>() {
        Ok(x) => x,
        Err(_) => i64::MAX,
    }
}

fn can_solve_with_concat(
    required_result: i64,
    array: &Vec<i64>,
    index: usize,
    running_result: i64,
) -> bool {
    if index == array.len() {
        return required_result == running_result;
    }
    if running_result > required_result {
        return false;
    }
    return can_solve_with_concat(
        required_result,
        array,
        index + 1,
        running_result + array[index],
    ) || can_solve_with_concat(
        required_result,
        array,
        index + 1,
        safe_mul(running_result, array[index]),
    ) || can_solve_with_concat(
        required_result,
        array,
        index + 1,
        get_concat(running_result, array[index]),
    );
}

pub fn part1(lines: Vec<String>) {
    let mut result = 0;
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

        if can_solve(required_result, &nums, 0, 0) {
            result += required_result;
        }
    }

    println!("{}", result);
}

pub fn part2(lines: Vec<String>) {
    let mut result = 0;
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

        if can_solve_with_concat(required_result, &nums, 0, 0) {
            result += required_result;
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
