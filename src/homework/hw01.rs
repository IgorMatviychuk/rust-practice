fn main() {
    const ENVELOPE_WIDTH: usize = 25;
    const ENVELOPE_HEIGHT: usize = 10;

    for y in 0..ENVELOPE_HEIGHT {
        for x in 0..ENVELOPE_WIDTH {
            let is_hor = y == 0 || y == ENVELOPE_HEIGHT - 1;
            let is_ver = x == 0 || x == ENVELOPE_WIDTH - 1;
            let is_diag1 = x == y * ENVELOPE_WIDTH / ENVELOPE_HEIGHT;
            let is_diag2 = x == (ENVELOPE_HEIGHT - 1 - y) * ENVELOPE_WIDTH / ENVELOPE_HEIGHT;

            let to_show = is_hor || is_ver || is_diag1 || is_diag2;

            let sym = if to_show { '*' } else { ' ' };
            print!("{}",sym);
        }
        println!();
    }
}
