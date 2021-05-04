use puzzle::latin::solve;
use std::io::{self, BufRead};

fn main() {
    // Input
    let stdio = io::stdin();
    let mut input = stdio.lock();
    let mut line = String::new();
    let mut matrix = vec![];
    while input.read_line(&mut line).unwrap() > 0 {
        matrix.push(
            line.trim_end_matches('\n')
                .as_bytes()
                .iter()
                .map(|x| *x - 0x30)
                .collect::<Vec<u8>>(),
        );
        line.clear();
    }

    // Solve
    solve(matrix);
}
