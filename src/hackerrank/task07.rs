// - https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true

use std::io;

fn mini_max_sum(arr: &[i64]) {
    let total_sum: i64 = arr.iter().sum();
    let min = arr.iter().max().unwrap();
    let max = arr.iter().min().unwrap();
    let min_sum = total_sum - min;
    let max_sum = total_sum - max;
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    if arr.len() == 5 {
        mini_max_sum(&arr);
    } else {
        eprintln!("Please enter exactly 5 integers.");
    }
}



