use utils::context::Context;

fn main() {
    let ctx = Context::new();
    let input = ctx.read_lines_or_exit();
    let total: i64 = input.lines().map(jolt).sum();
    println!("Sum is {}", total);
}

fn jolt(line: &str) -> i64 {
    let trimmed = line.trim();
    if line.is_empty() {
        return 0;
    }
    // print!("{}", trimmed);
    let mut result = 0;
    let mut multiplicator = 10_00000_00000;

    // Stores char, position
    let mut chars: Vec<(char, usize)> = trimmed.char_indices().map(|(i, c)| (c, i)).collect();
    chars.sort_by(|&(c1, i1), &(c2, i2)| c2.cmp(&c1).then_with(|| i1.cmp(&i2)));
    for space in (1..=12).rev() {
        let max_idx = trimmed.len() - space;
        let (highest, idx) = chars
            .iter()
            .skip_while(|(_, i)| *i > max_idx)
            .next()
            .expect("ur code is wrong");
        // Drop all chars before highest
        result += (*highest as i64 - '0' as i64) * multiplicator;
        multiplicator /= 10;
        chars = chars.iter().copied().filter(|(_, i)| i > idx).collect();
    }
    // println!(" -> {}", result);
    result
}
