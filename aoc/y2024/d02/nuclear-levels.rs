use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(input) = read_input() {
        let mut res : i32 = 0;
        'outer: for line in input {
            match line {
                Ok(l) => {
                    let first = l.split(" ").take(1).next();
                    let mut increase = false;
                    let mut decrease = false;
                    let mut prev : i32;
                    match first {
                       Some(x) => {
                        prev = x.parse().expect("Could not parse int!")
                    }
                        _ => continue
                    }
                    for number in l.split(" ").skip(1) {
                        let next : i32 = number.parse().expect("Integer");
                        if next > prev {
                            increase = true;
                        }
                        if prev > next {
                            decrease = true;
                        }
                        if (next - prev).abs() > 3 || next == prev || (increase && decrease) {
                            continue 'outer;
                        } else {
                            prev = next;
                        }
                    }
                    res += 1;  
                }
                _ => panic!("Invalid format!"),
            }
        }
        println!("There are {:?} safe reports!", res);
    }  else {
        panic!("Could not read file!");
    }

}


fn read_input() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("d02/input.txt")?;
    Ok(io::BufReader::new(file).lines())
}
