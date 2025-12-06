use std::str::Chars;

use utils::{context::Context, verbose, verboseln};

fn main() {
    let input = Context::get().read_lines_or_exit();
    let lines: Vec<&str> = input.lines().collect();
    // Divide input into operator and line iterators
    let mut line_iterators: Vec<Chars> =
        lines[..lines.len() - 1].iter().map(|l| l.chars()).collect();
    let mut ops = lines.last().expect("Bad input").chars().peekable();
    let mut op = ops.next();
    let mut sum = 0;
    // Read input for one operation
    while op.is_some() {
        let mut next_op: Option<char> = None;
        // Count number of numbers
        let mut n_nums = ops
            .by_ref()
            .take_while(|c| {
                next_op = Some(*c);
                c.is_whitespace()
            })
            .count();
        if ops.peek() == None { // EOL
            n_nums += 1; // Add missing separator space
            next_op = None; // Break loop
        }
        verbose!("Operator {} - {} numbers", op.unwrap(), n_nums);
        // Build transposed numbers
        let mut nums: Vec<u64> = vec![0; n_nums];
        let mut multiplicators: Vec<u64> = vec![1; n_nums];
        for it in line_iterators.iter_mut().rev() {
            for ((digit, num), mult) in it
                .take(n_nums)
                .map(|c| c.to_digit(10))
                .zip(nums.iter_mut())
                .zip(multiplicators.iter_mut())
            {
                if let Some(n) = digit {
                    *num += n as u64 * *mult;
                    *mult *= 10;
                }
            }
            it.next(); // Skip whitespace
        }
        verbose!(" {:?}", nums);
        let result: u64 = match op.unwrap() {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0
        };
        verboseln!(" = {}", result);
        sum += result;
        op = next_op;
    }
    println!("Result is {}", sum);
}
