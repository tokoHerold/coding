use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_input() {

        let mut left: Vec<i32> = vec![];
        let mut right : HashMap<i32, i32> = HashMap::new();

        for line in lines {
            if let Ok(text) = line {
                let entries : Vec<&str> = text.split("   ").collect();
                let l : i32 = entries[0].parse().expect("Invalid number");
                let r : i32 = entries[1].parse().expect("Invalid number"); 
                left.push(l);
                if let Some(counter) = right.get_mut(&r) {
                    *counter = *counter + 1;
                } else {
                    right.insert(r, 1);
                };
            }
        }
        
        let mut similarity = 0;
        for l in left {
            if let Some(r) = right.get(&l) {
                similarity += l * r;
            }
        }

        println!("The similarity score is {}", similarity);

    } else {
        panic!("Failed to read file!");
    }

}

fn read_input() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input.txt")?;
    Ok(io::BufReader::new(file).lines())
}