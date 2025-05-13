//- https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true

use std::io;

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;
        let line = " ".repeat(spaces as usize) + &"#".repeat(hashes as usize);
        println!("{}", line);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Input is not a valid number");

    if n > 0 && n <= 100 {
        staircase(n);
    } else {
        eprintln!("n must be between 1 and 100");
    }
}


