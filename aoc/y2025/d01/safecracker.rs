use std::{env, fs, process};

fn main() {
    print!("Hello world!\n");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-file>", &args[0]);
        process::exit(1)
    }
    let path = &args[1];
    match fs::read_to_string(path) {
        Err(error) => {
            eprintln!("Error: could not open {}: {}", path, error);
        }
        Ok(content_str) => {
            let mut code = 0;
            let mut position = 50;
            for mut line in content_str.lines() {
                line = line.trim();
                if line.is_empty() {
                    continue;
                };
                match line.chars().next() {
                    Some('L') => {
                        if let Ok(num) = &line[1..].parse::<i32>() {
                            position = ((position - num) % 100 + 100) % 100;
                        } else {
                            eprintln!("Bad format: {}", line);
                            process::exit(1);
                        }
                    }
                    Some('R') => {
                        if let Ok(num) = &line[1..].parse::<i32>() {
                            position = (position + num) % 100;
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
                println!("Position: {}", position);
                if position == 0 {
                    code += 1;
                }
            }
            println!("The code is {}.", code);
        }
    }
}
