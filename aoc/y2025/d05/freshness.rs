use std::cmp::{max, min};

use regex::Regex;
use utils::{context::Context, verboseln};

fn insert_range(vec: &mut Vec<(i64, i64)>, lower: i64, upper: i64) {
    let mut pos = -1;
    for (i, (l, h)) in vec.iter_mut().enumerate() {
        if upper < *l {
            continue;
        }
        if lower > *h {
            pos = i as i32;
            break;
        }
        *l = min(*l, lower);
        *h = max(*h, upper);
        return;
    }
    vec.insert((pos + 1) as usize, (lower, upper));
}

fn main() {
    let input = Context::get().read_lines_or_exit();
    let mut input_lines = input.lines().peekable();
    let mut result = 0;
    let re = Regex::new(r"(?<low>[0-9]+)-(?<high>[0-9]+)*").expect("Regex wrong");
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for line in &mut input_lines {
        if line.is_empty() {
            break;
        }
        let capture = re.captures(line).expect("Bad input");
        let lower = capture
            .name("low")
            .expect("Bad input")
            .as_str()
            .parse::<i64>()
            .expect("Bad input");
        let upper = capture
            .name("high")
            .expect("Bad input")
            .as_str()
            .parse::<i64>()
            .expect("Bad input");
        insert_range(&mut ranges, lower, upper);
    }

    if Context::get().is_verbose() {
        for (x, y) in ranges.iter() {
            println!("Range: {} - {}", x, y);
        }
    }

    for line in input_lines {
        let val: i64 = line.parse::<i64>().expect("Bad input");
        for (l, u) in ranges.iter() {
            if *l <= val && val <= *u {
                verboseln!("{} is fresh", val);
                result += 1;
                break;
            }
        }
    }
    println!("Result is {}", result);
}
