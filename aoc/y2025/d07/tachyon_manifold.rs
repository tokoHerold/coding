use utils::context::Context;

fn main() {
    let input = Context::get().read_lines_or_exit();
    let mut lines = input.lines();

    let mut beams: Vec<bool> = lines.next().expect("No input").chars().map(|c| c == 'S').collect();
    let mut beams_shadow = vec![false; beams.len()];
    let mut splits = 0;

    if Context::get().is_verbose() {
        println!( "{:?}", beams.iter().map(|b| if *b { '|' } else { '.' }).collect::<String>());
    }
    lines.for_each(|l| {
        for (i, (c, b)) in l.chars().zip(beams.iter_mut()).enumerate() {
            if *b {
                if c == '.' {
                    beams_shadow[i] = true;
                } else {
                    splits += 1;
                    *b = false;
                    beams_shadow[i - 1] = true;
                    beams_shadow[i] = false;
                    beams_shadow[i + 1] = true;
                }
            }
        }
        std::mem::swap(&mut beams, &mut beams_shadow);
        if Context::get().is_verbose() {
            println!( "{:?}", beams.iter().map(|b| if *b { '|' } else { '.' }).collect::<String>());
        }
    });
    println!("Number of splits: {}", splits);
}
