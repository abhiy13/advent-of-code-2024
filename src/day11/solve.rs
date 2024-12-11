use std::collections::HashMap;

use crate::reader::reader;
use anyhow::Result;

fn get_split(num: i128, level: i32, cache: &mut HashMap<(i128, i32), i128>) -> i128 {
    if level == 0 {
        return 1;
    }

    if cache.contains_key(&(num, level)) {
        return *cache.get(&(num, level)).unwrap();
    }

    let mut result = 0;

    if num == 0 {
        result = get_split(1, level - 1, cache);
    } else if num.to_string().len() % 2 == 0 {
        let st = num.to_string();
        result = get_split(
            st[..st.len() / 2].parse::<i128>().unwrap(),
            level - 1,
            cache,
        );
        result += get_split(
            st[st.len() / 2..].parse::<i128>().unwrap(),
            level - 1,
            cache,
        );
    } else {
        result += get_split(num * 2024, level - 1, cache);
    }
    cache.insert((num, level), result);
    return result;
}

pub fn solve() -> Result<()> {
    let lines = reader::read();

    let input: Vec<i128> = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    let mut result1: i128 = 0;
    let mut result2: i128 = 0;
    let mut cache = HashMap::<(i128, i32), i128>::new();

    for x in input {
        result1 += get_split(x, 25, &mut cache);
        result2 += get_split(x, 75, &mut cache);
    }

    println!("Part1: {}", result1);
    println!("Part2: {}", result2);

    Ok(())
}
