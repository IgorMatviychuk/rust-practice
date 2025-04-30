//https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true

use std::io;

fn main() {
    let mut n_line = String::new();
    io::stdin().read_line(&mut n_line).expect("Failed to read line");

    let mut arr_line = String::new();
    io::stdin().read_line(&mut arr_line).expect("Failed to read line");

    let arr: Vec<i32> = arr_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let sum: i32 = arr.iter().sum();
    println!("{}", sum);
}
