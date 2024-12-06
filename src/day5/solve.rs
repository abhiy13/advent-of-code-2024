use std::collections::{HashMap, HashSet};

use crate::reader::reader;
use anyhow::Result;

pub fn is_correct(graph: &HashMap<i32, Vec<i32>>, order: &Vec<i32>) -> bool {
    let mut sat = true;
    for i in 0..order.len() {
        if !graph.contains_key(&order[i]) {
            continue;
        }
        let this_map = graph.get(&order[i]).unwrap();
        for j in 0..i {
            sat &= !this_map.contains(&order[j]);
        }
    }
    // println!("sat: {}, for: {:?}, graph: {:?}", sat, order, graph);
    return sat;
}

// brute
pub fn part1(lines: Vec<String>) {
    let mut map = HashMap::<i32, Vec<i32>>::new();
    let mut iter = lines.iter();
    loop {
        let line = iter.next().unwrap();
        if line == "" {
            break;
        }
        let nums = line
            .split("|")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if !map.contains_key(&nums[0]) {
            map.insert(nums[0].into(), Vec::new());
        }
        map.get_mut(&nums[0]).unwrap().push(nums[1]);
    }

    let mut result = 0;

    while let Some(line) = iter.next() {
        let nums = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        result += if is_correct(&map, &nums) {
            nums[nums.len() / 2]
        } else {
            0
        };
    }
    println!("{}", result);
}

fn topological_sort(
    node: i32,
    graph: &HashMap<i32, Vec<i32>>,
    stack: &mut Vec<i32>,
    visited: &mut HashSet<i32>,
) {
    visited.insert(node);
    for children in graph.get(&node).unwrap_or(&vec![]) {
        if !visited.contains(children) {
            topological_sort(*children, graph, stack, visited);
        }
    }

    stack.push(node);
}

pub fn part2(lines: Vec<String>) {
    let mut graph = HashMap::<i32, Vec<i32>>::new();
    let mut iter = lines.iter();
    loop {
        let line = iter.next().unwrap();
        if line == "" {
            break;
        }
        let nums = line
            .split("|")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if !graph.contains_key(&nums[0]) {
            graph.insert(nums[0], Vec::new());
        }
        graph.get_mut(&nums[0]).unwrap().push(nums[1]);
    }

    let mut sum = 0;

    while let Some(line) = iter.next() {
        let nums = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_correct(&graph, &nums) {
            continue;
        }

        let mut visited = HashSet::<i32>::new();
        let nums_set = nums.clone().into_iter().collect::<HashSet<i32>>();
        let mut array = Vec::new();

        let new_graph = graph
            .clone()
            .into_iter()
            .filter(|(node, _)| nums_set.contains(node))
            .map(|(k, v)| (k, v.into_iter().filter(|x| nums_set.contains(x)).collect()))
            .collect::<HashMap<i32, Vec<i32>>>();

        for num in nums.iter() {
            if visited.contains(num) {
                continue;
            }
            topological_sort(*num, &new_graph, &mut array, &mut visited);
        }

        array.reverse();
        sum += array[array.len() / 2];
    }

    println!("{}", sum);
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    // part1(lines);
    part2(lines);

    Ok(())
}
