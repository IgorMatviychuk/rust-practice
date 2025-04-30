//- https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true

use std::io;

fn plus_minus(arr: Vec<i32>) {
    let n = arr.len() as f64;
    let positive = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negative = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zero = arr.iter().filter(|&&x| x == 0).count() as f64;
    
    println!("{:.6}", positive / n);
    println!("{:.6}", negative / n);
    println!("{:.6}", zero / n);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    let mut arr = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let elements: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please enter a valid integer"))
        .collect();

    arr = elements;

    plus_minus(arr);
}
