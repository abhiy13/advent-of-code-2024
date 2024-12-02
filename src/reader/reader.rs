use std::io;
use std::io::BufRead;

/// Reads input from console and returns it as a vec of lines
pub fn read() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut out: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        match line {
            Ok(x) => {
               out.push( x);
            }
            _ => {}
        };
    }

    out
}