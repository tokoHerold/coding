use utils::context;

fn main() {
    let ctx = context::Context::new();
    let input = ctx.read_lines_or_exit();
    let id_ranges = input.split(',');
    let mut sum: i64 = 0;
    for mut range in id_ranges {
        range = range.trim();
        let delim_idx = range.find('-').expect("Bad input");
        let (lower_str, upper_str) = range.split_at(delim_idx);
        let lower = lower_str.parse::<i64>().expect("Bad input");
        let upper = upper_str[1..].parse::<i64>().expect("Bad input");
        for id in lower..=upper {
            if valid_id(id) {
                continue;
            }
            ctx.verbose(format_args!("Bad id: {}", id));
            sum += id;
        }
    }
    println!("Sum of ids = {}", sum);
}

fn valid_id(id: i64) -> bool {
    let decimal = id.to_string();
    if decimal.len() % 2 == 1 {
        return true;
    }
    let middle = decimal.len() / 2;
    let (first, second) = decimal.split_at(middle);
    first != second
}

