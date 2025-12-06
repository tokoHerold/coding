use utils::{context::Context, verbose, verboseln};

fn main() {
    let input = Context::get().read_lines_or_exit();
    let lines: Vec<&str> = input.lines().collect();
    let operations: Vec<char> = lines
        .last()
        .expect("Bad input")
        .split_whitespace()
        .map(|str| str.chars().next().expect("Bad input"))
        .collect();
    verboseln!("Operations: {:?}", &operations);
    let mut results: Vec<u64> = operations
        .iter()
        .map(|op| if *op == '*' { 1 } else { 0 })
        .collect();
    for line in &lines[..lines.len() - 1] {
        for ((num, op), res) in line
            .split_whitespace()
            .map(|str| str.parse::<u64>().expect("Bad input"))
            .zip(operations.iter())
            .zip(results.iter_mut())
        {
            match op {
                '+' => *res += num,
                '*' => *res *= num,
                _ => panic!("Bad input"),
            }
            verbose!("{} ", num);
        }
        verboseln!("");
    }
    verboseln!("{:?}", results);
    println!("Result is {}", results.iter().sum::<u64>());
}
