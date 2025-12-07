use utils::context::Context;

fn main() {
    let input = Context::get().read_lines_or_exit();
    let mut lines = input.lines();

    let mut beams: Vec<u64> = lines .next() .expect("No input") .chars() .map(|c| if c == 'S' { 1 } else { 0 }) .collect();
    let mut beams_shadow: Vec<u64> = vec![0; beams.len()];

    lines.for_each(|l| {
        beams_shadow.fill(0);
        for (i, (c, n)) in l.chars().zip(beams.iter_mut()).enumerate() {
            if *n > 0 {
                if c == '.' {
                    beams_shadow[i] += *n;
                } else {
                    beams_shadow[i - 1] += *n;
                    beams_shadow[i] = 0;
                    beams_shadow[i + 1] += *n;
                    *n = 0;
                }
            }
        }
        std::mem::swap(&mut beams, &mut beams_shadow);
    });
    println!("Number of timelines: {}", beams.iter().map(|n| *n as usize).sum::<usize>());
}
