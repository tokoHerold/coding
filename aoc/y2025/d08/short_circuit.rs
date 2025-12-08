use std::collections::{BTreeSet, HashSet};
use std::{process, time};

use utils::context::Context;
use utils::geometry::Vec3;

fn main() {
    let start = time::Instant::now();
    // Read input
    let input = Context::get().read_lines_or_exit();
    let lines = input.lines();
    let junctions: Vec<Vec3<i64>> = lines
        .map(|l| {
            let vals: Vec<i64> = l
                .split(',')
                .map(|s| s.parse().expect("Bad input"))
                .collect();
            Vec3::new(vals[0], vals[1], vals[2])
        })
        .collect();

    // Calculate distance between all junctions
    let mut distances_to: BTreeSet<(i64, usize, usize)> = BTreeSet::new();
    for (idx_a, junction_a) in junctions.iter().enumerate() {
        for idx_b in idx_a + 1..junctions.len() {
            let junction_b = &junctions[idx_b];
            let d = junction_a.distance_squared(junction_b);
            distances_to.insert((d, idx_a, idx_b));
        }
    }

    // Connect circuits
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for (_, junc_idx_a, junc_idx_b) in distances_to.iter() {
        let mut set_a: Option<usize> = None;
        let mut set_b: Option<usize> = None;
        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(junc_idx_a) {
                set_a = Some(idx);
            }
            if circuit.contains(junc_idx_b) {
                set_b = Some(idx);
            }
            if set_a.is_some() && set_b.is_some() {
                break;
            }
        }
        if let Some(a) = set_a {
            if let Some(b) = set_b {
                if a != b { // Not on same circuit
                    let circuit_b = circuits[b].clone();
                    let circuit_a = &mut circuits[a];
                    circuit_a.extend(circuit_b); // Merge circuits
                    circuits.swap_remove(b);
                }
            } else { // b is None
                circuits[a].insert(*junc_idx_b);
            }
        } else if let Some(b) = set_b { // a is None
            circuits[b].insert(*junc_idx_a);
        } else { // a and b are None
            let mut set: HashSet<usize> = HashSet::new();
            set.insert(*junc_idx_a);
            set.insert(*junc_idx_b);
            circuits.push(set);
        }
        // Check if done
        if circuits.len() == 1 && circuits[0].len() == junctions.len() {
            let ja = &junctions[*junc_idx_a];
            let jb = &junctions[*junc_idx_b];
            println!("Last connection: {:?}, {:?} (x coords multiplied: {})", ja, jb, ja.x * jb.x);
            println!("Took {:?}", start.elapsed());
            process::exit(0);
        }
    }
}
