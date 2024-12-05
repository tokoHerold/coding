use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(input) = read_input() {
        let mut res : i32 = 0;
        for line in input {
            match line {
                Ok(l) => {

                    let numbers : Vec<i32> = l.split(" ").map(|x| {
                    let n : i32 = x.parse().expect("Could not parse int!");
                    n 
                }).collect();

                    {
                        let violations = check_monotony(&numbers, false); 
                        let success = dampen(&numbers, violations, false);
                        if success {
                            res += 1;
                            continue;
                        }
                        // print!(" - dec {}", success);
                    } {
                        let violations = check_monotony(&numbers, true); 
                        let success = dampen(&numbers, violations, true); 
                        if success {
                            res += 1;
                            continue;
                        }
                        // print!(" - inc {}", success);
                    }
                    //println!("{:?}", numbers); 
                }
                _ => panic!("Invalid format!"),
            }
        }
        println!("There are {:?} safe reports!", res);
    }  else {
        panic!("Could not read file!");
    }

}

fn dampen(numbers : &[i32], violations : HashSet<i32>, increasing : bool) -> bool {
    if violations.is_empty() { return true; }
    if violations.len() > 3 { return false; }
    for idx in violations.iter() {
        let mut copy : Vec<i32> = numbers.to_vec();
        copy.remove(*idx as usize);
        let new_violations = check_monotony(&copy, increasing); 
        if new_violations.is_empty() {
            return true;
        }
    }
    false
}

fn check_monotony(numbers : &[i32], increasing : bool) -> HashSet<i32> {
    let mut vioalting_ids : HashSet<i32> = HashSet::new();
    for (i, number) in numbers.iter().skip(1).enumerate() {
        if let Some(prev) = numbers.get(i) {
            let mut diff = number - prev;
            if !increasing {
                diff = -diff;
            }
            if diff <= 0 || diff > 3 {
                let idx : i32 = i.try_into().expect("");
                vioalting_ids.insert(idx);
                vioalting_ids.insert(idx + 1);
            }
            
        } else {
            continue;
        }
    }

    vioalting_ids
}


fn read_input() -> io::Result<io::Lines<io::BufReader<File>>> {
    // let file = File::open("d02/example_input.txt")?;
    let file = File::open("d02/input.txt")?;
    Ok(io::BufReader::new(file).lines())
}
