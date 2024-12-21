use std::{cmp, i64};

use crate::reader::reader;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
struct Button {
    dx: i64,
    dy: i64,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    x: i64,
    y: i64,
    a: Button,
    b: Button,
}

fn parse_input(lines: Vec<String>) -> Vec<Game> {
    let mut response = Vec::new();

    let get_positions = |str: &String| -> (i64, i64) {
        let f = str.split(",").into_iter().collect::<Vec<&str>>();
        let x = f[0].split("X").collect::<Vec<&str>>()[1][1..]
            .parse::<i64>()
            .unwrap();
        let y = f[1][3..].parse::<i64>().unwrap();
        (x, y)
    };

    for i in (0..lines.len()).step_by(4) {
        let (mut x, mut y) = get_positions(&lines[i]);
        let a = Button { dx: x, dy: y };
        (x, y) = get_positions(&lines[i + 1]);
        let b = Button { dx: x, dy: y };
        (x, y) = get_positions(&lines[i + 2]);
        let game = Game { x, y, a, b };
        response.push(game);
    }

    response
}

fn solve_one(game: &Game) -> i64 {
    let mut min = i64::MAX;
    for i in 0..=100 {
        for j in 0..=100 {
            let fx = i * game.a.dx + j * game.b.dx;
            let fy = i * game.a.dy + j * game.b.dy;
            if fx == game.x && fy == game.y {
                min = cmp::min(min, 3 * i + j);
            }
        }
    }
    if min == i64::MAX {
        0
    } else {
        min
    }
}

fn solve_one_math(game: &Game) -> i64 {
    let (fx, fy) = (game.x, game.y);
    let n = fy * game.b.dx - fx * game.b.dy;
    let d = game.a.dy * game.b.dx - game.a.dx * game.b.dy;

    if d == 0 || n % d != 0 {
        return 0;
    }

    let i = n / d;
    let j_numerator = fy - i * game.a.dy;

    if j_numerator % game.b.dy != 0 {
        return 0;
    }

    let j = j_numerator / game.b.dy;
    if i < 0 || j < 0 {
        return 0;
    }
    return 3 * i + j;
}

pub fn solve() -> Result<()> {
    let lines = reader::read();
    let offset = 10000000000000;

    let games = parse_input(lines);
    let mut result = 0;
    let mut result2 = 0;

    for game in games {
        result += solve_one_math(&game);
        let game_new = Game {
            x: game.x + offset,
            y: game.y + offset,
            a: game.a,
            b: game.b,
        };
        result2 += solve_one_math(&game_new);
    }

    println!("Part1: {}", result);
    println!("Part2: {}", result2);
    Ok(())
}
