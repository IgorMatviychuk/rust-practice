//https://www.hackerrank.com/challenges/drawing-book/problem?isFullScreen=true

use std::io;

fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2; 
    let from_back = (n / 2) - (p / 2); 
    
    if n % 2 == 0 {
        let from_back_adjusted = if p == n { 0 } else { from_back };
        std::cmp::min(from_front, from_back_adjusted)
    } else {
        std::cmp::min(from_front, from_back)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let p: i32 = input.trim().parse().unwrap();

    println!("{}", page_count(n, p));
}
