use std::iter::zip;
use std::fs;

fn main() {

    let input : Vec<String> = fs::read_to_string("input.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut left : Vec<i32> = vec![];
    let mut right : Vec<i32> = vec![];

    for line in input {
        let entries : Vec<&str> = line.split("   ").collect();
        let l : i32 = entries[0].parse().expect("NaN"); 
        let r : i32 = entries[1].parse().expect("NaN");
        left.push(l);
        right.push(r);
    }

    assert!(left.len() == right.len());
    left.sort();
    right.sort();
    let mut distance : i32 = 0;

    for (i, j) in zip(left, right) {
        let d : i32 = i - j;
        distance += d.abs()
    }

    println!("Distance is {:?}", distance);
}
