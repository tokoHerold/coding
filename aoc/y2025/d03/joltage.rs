use std::collections::{BTreeMap, BTreeSet};
use utils::{context::Context, verbose, verboseln};

fn main() {
    let ctx = Context::get();
    let input = ctx.read_lines_or_exit();
    let total: i64 = input.lines().map(jolt).sum();
    println!("Sum is {}", total);
}

fn jolt(line: &str) -> i64 {
    let trimmed = line.trim();
    verbose!("{}", trimmed);
    if line.is_empty() {
        return 0;
    }
    // Add char -> index mapping
    let mut chars: BTreeMap<char, usize> = BTreeMap::new();
    for (i, c) in trimmed.char_indices() {
        if chars.len() >= 9 {
            break;
        }
        chars.entry(c).or_insert(i);
    }
    for (c, i) in chars.iter().rev() {
            if *i + 1 == trimmed.len() {
            continue;
        }
        // Find next highest char
        let rem = &trimmed[*i+1..];
        let rem_chars: BTreeSet<char> = rem.chars().collect();
        let x = 10 * (*c  as i64 - '0' as i64) + (*rem_chars.last().unwrap() as i64 - '0' as i64);
        verboseln!(" -> {}", x);
        return x;
    }
    return 0;
}
