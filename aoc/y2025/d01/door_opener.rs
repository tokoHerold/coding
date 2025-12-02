use std::process;

use utils::context;

fn main() {
    let ctx = context::Context::new();
    let input = ctx.read_lines_or_exit();
    let mut position = 50;
    let mut code = 0;
    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        };
        match line.chars().next() {
            Some('L') => {
                if let Ok(num) = &line[1..].parse::<i32>() {
                    let added = position - num;
                    let full_rotations = added / -100;
                    code += full_rotations;
                    let partial_rotation = added % 100;
                    if partial_rotation <= 0 && position != 0 {
                        code += 1;
                    }
                    position = (partial_rotation + 100) % 100;
                } else {
                    eprintln!("Bad format: {}", line);
                    process::exit(1);
                }
            }
            Some('R') => {
                if let Ok(num) = &line[1..].parse::<i32>() {
                    let added = position + num;
                    let full_rotations = added / 100;
                    let partial_rotation = added % 100;

                    if partial_rotation >= 100 {
                        code += 1;
                    }

                    code += full_rotations;
                    position = partial_rotation;
                } else {
                    eprintln!("Bad format: {}", line);
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Bad format: {}", line);
                process::exit(1);
            }
        }
        ctx.verbose(format_args!("{} Pos: {} - Code: {}", line, position, code));
    }
    println!("The code is {}.", code);
}
