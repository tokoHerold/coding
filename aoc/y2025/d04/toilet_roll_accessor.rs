use std::collections::HashSet;

use utils::context::Context;

fn main() {
    let ctx = Context::get();
    let input = ctx.read_lines_or_exit();
    let mut field: HashSet<(i32, i32)> = HashSet::new();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.trim().char_indices() {
            if c == '@' {
                field.insert((i as i32, j as i32));
            }
        }
    }

    let count: usize = field
        .iter()
        .filter(|(x, y)| {
            let i = *x;
            let j = *y;
            vec![(i-1, j-1), (i, j-1), (i+1, j-1), (i-1, j), (i+1, j), (i-1, j+1), (i, j+1), (i+1, j+1)]
                .iter()
                .filter(|t| field.contains(t))
                .count() < 4
        })
        .count();
    println!("Result is {}", count);
}
