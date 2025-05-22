//https://www.hackerrank.com/challenges/the-birthday-bar/problem?isFullScreen=true

fn birthday(s: &[i32], d: i32, m: usize) -> i32 {
    let mut count = 0;
    for i in 0..=s.len().saturating_sub(m) {
        let sum: i32 = s[i..i + m].iter().sum();
        if sum == d {
            count += 1;
        }
    }
    count
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let s: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let parts: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let d = parts[0]; 
    let m = parts[1] as usize; 

    let result = birthday(&s, d, m);
    println!("{}", result);
}
