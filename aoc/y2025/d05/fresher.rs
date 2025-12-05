#![feature(linked_list_cursors)]

use std::{
    cmp::{max, min},
    collections::LinkedList,
};

use regex::Regex;
use utils::{context::Context, verboseln};

fn insert_range(list: &mut LinkedList<(i64, i64)>, mut lower: i64, mut upper: i64) {
    let mut cursor = list.cursor_front_mut();
    while let Some((l, h)) = cursor.current() {
        if *h < lower {
            cursor.move_next();
            continue;
        }
        if *l > upper {
            break;
        }
        lower = min(lower, *l);
        upper = max(upper, *h);
        cursor.remove_current();
    }
    cursor.insert_before((lower, upper));
}

fn main() {
    let input = Context::get().read_lines_or_exit();
    let mut input_lines = input.lines().peekable();
    let mut result: usize = 0;
    let re = Regex::new(r"(?<low>[0-9]+)-(?<high>[0-9]+)*").expect("Regex wrong");
    let mut ranges: LinkedList<(i64, i64)> = LinkedList::new();
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

    for (x, y) in ranges.iter() {
        result += (y - x) as usize;
        verboseln!("Range: {} - {}", x, y);
    }
    result += ranges.len();
    println!("Result is {}", result);
}
