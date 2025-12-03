use std::collections::HashSet;

use utils::context::Context;

fn main() {
    let ctx = Context::get();
    let input = ctx.read_lines_or_exit();
    let id_ranges = input.split(',');
    let mut sum: i64 = 0;
    for mut range in id_ranges {
        range = range.trim();
        let delim_idx = range.find('-').expect("Bad input");
        let (lower_str, upper_str) = range.split_at(delim_idx);
        let lower = lower_str.parse::<i64>().expect("Bad input");
        let upper = upper_str[1..].parse::<i64>().expect("Bad input");
        for id in lower..=upper {
            if valid_id(id) {
                continue;
            }
            ctx.verbose(format_args!("Bad id: {}", id));
            sum += id;
        }
    }
    println!("Sum of ids = {}", sum);
}

fn factor(number: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();
    for i in 2..number {
        if number % i == 0 {
            factors.push(number / i);
        }
    }
    factors
}

fn valid_id(id: i64) -> bool {
    let decimal = id.to_string();
    let n = decimal.len();
    if n == 1 {
        return true;
    }
    let chars: HashSet<char> = decimal.chars().collect();
    if chars.len() == 1 {
        return false;
    }
    'outer: for split_len in factor(n) {
        let first = &decimal[..split_len];
        for i in (split_len..n).step_by(split_len) {
            let second = &decimal[i..i + split_len];
            if first != second {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
