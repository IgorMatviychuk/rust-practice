fn main() {
    const W: usize = 11;
    const H: usize = 11;

    for y in 0..H {
        for x in 0..W {
            let ch = if x >= (W / 2 - y.min(H - y - 1)) && x <= (W / 2 + y.min(H - y - 1)) {
                '*'
            } else {
                ' '
            };
            print!("{}", ch);
        }
        println!();
    }
}
