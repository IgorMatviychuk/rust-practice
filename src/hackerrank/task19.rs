//https://www.hackerrank.com/challenges/sock-merchant/problem?isFullScreen=true

use std::io;

fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut frequency = vec![0; 101]; 
    for &color in ar {
        frequency[color as usize] += 1;
    }
    
    let mut pairs = 0;
    for count in frequency {
        pairs += count / 2; 
    }
    
    pairs
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", sock_merchant(n, &ar));
}
