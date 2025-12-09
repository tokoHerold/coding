use std::collections::BTreeSet;
use std::cmp::max;
use std::time;

use utils::context::Context;
use utils::verboseln;

fn main() {
    let start = time::Instant::now();
    let input = Context::get().read_lines_or_exit();
    let n = Context::get_number() as usize;
    let lines = input.lines();
    let mut nodes: Vec<(i32, i32)> = Vec::with_capacity(lines.clone().count());
    let mut diag1: BTreeSet<(i32, usize)> = BTreeSet::new(); // Manhattan distance to origin
    let mut diag2: BTreeSet<(i32, usize)> = BTreeSet::new(); // Manhatten distance to upper right corner
    let mut x_max = 0;

    // Read input and sort by |1| distance to top corners
    for (i, line) in lines.enumerate() {
        let mut split = line.split(',');
        let x: i32 = split.next().expect("Bad input").parse().expect("Bad input");
        let y: i32 = split.next().expect("Bad input").parse().expect("Bad input");
        x_max = max(x, x_max);
        nodes.push((x, y));
        diag1.insert((x + y, i));
    }
    for (i, (x, y)) in nodes.iter().enumerate() {
        diag2.insert((x_max - x + y, i));
    }

    // Get all elements which are closes to each corner
    // let (smallest, _) = diag1.iter().next().unwrap();
    // let nw: Vec<usize> = diag1.iter().take_while(|(d, _)| d == smallest).map(|(_, i)| *i).collect();
    // let (largest, _) = diag1.iter().last().unwrap();
    // let se: Vec<usize> = diag1.iter().rev().take_while(|(d, _)| d == largest).map(|(_, i)| *i).collect();
    // let (smallest, _) = diag2.iter().next().unwrap();
    // let ne: Vec<usize> = diag2.iter().take_while(|(d, _)| d == smallest).map(|(_, i)| *i).collect();
    // let (largest, _) = diag2.iter().last().unwrap();
    // let sw: Vec<usize> = diag2.iter().rev().take_while(|(d, _)| d == largest).map(|(_, i)| *i).collect();
    let nw: Vec<usize> = diag1.iter().take(n).map(|(_, i)| *i).collect();
    let se: Vec<usize> = diag1.iter().rev().take(n).map(|(_, i)| *i).collect();
    let ne: Vec<usize> = diag2.iter().take(n).map(|(_, i)| *i).collect();
    let sw: Vec<usize> = diag2.iter().rev().take(n).map(|(_, i)| *i).collect();
    verboseln!("Trying to find largest regions for these candidates:\n\tDiag1: {:?} - {:?}\n\tDiag2: {:?} - {:?}", nw, se, ne, sw);

    // Iterate over all candidates to find largest area
    let (mut max_area, mut idx1, mut idx2): (u64, usize, usize) = (0, 0, 0);
    for (corner_1, corner_2) in [(nw, se), (ne, sw)] {
        for idx_a in &corner_1 {
            for idx_b in &corner_2 {
                let (x1, y1) = nodes[*idx_a];
                let (x2, y2) = nodes[*idx_b];
                let area = ((x2 - x1).abs() + 1) as u64 * ((y2 - y1).abs() + 1) as u64;
                if area >= max_area {
                    max_area = area;
                    idx1 = *idx_a;
                    idx2 = *idx_b;
                    verboseln!("Found new largest area between {:?} and {:?}: {}", nodes[idx1], nodes[idx2], max_area);
                }
            }
        }
    }
    println!("The largest area is between {:?} and {:?} - it is {}", nodes[idx1], nodes[idx2], max_area);
    println!("Took {:?}", start.elapsed());
}
