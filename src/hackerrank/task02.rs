//https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true

use std::io;

fn main() {
    let mut a_line = String::new();
    let mut b_line = String::new();

    io::stdin().read_line(&mut a_line).expect("Failed to read line");
    let a: Vec<i32> = a_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    io::stdin().read_line(&mut b_line).expect("Failed to read line");
    let b: Vec<i32> = b_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    println!("{} {}", alice_score, bob_score);
}
