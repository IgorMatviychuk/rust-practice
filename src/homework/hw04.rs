fn draw_christmas_tree(num_segments: usize) {
    let last_segment = num_segments - 1;
    let last_height = if last_segment == 0 { 3 } else { last_segment + 2 };
    let max_width = 2 * (last_height - 1) + 1;

    for i in 0..num_segments {
        let height = if i == 0 { 3 } else { i + 2 };
        for j in 0..height {
            let stars = if i == 0 {
                match j {
                    0 | 1 => 1,
                    _ => 3,
                }
            } else {
                2 * j + 1
            };
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }
}

fn main() {
    let num_segments = 5;
    draw_christmas_tree(num_segments);
}
