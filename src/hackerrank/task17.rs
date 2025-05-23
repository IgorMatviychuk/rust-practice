//https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true

use std::io;

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut frequency = [0; 6]; 
    for &bird in arr {
        frequency[bird as usize] += 1;
    }
    
    let mut max_freq = 0;
    let mut min_type = 5;
    for i in 1..=5 {
        if frequency[i] > max_freq {
            max_freq = frequency[i];
            min_type = i;
        } else if frequency[i] == max_freq && i < min_type {
            min_type = i;
        }
    }
    
    min_type as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", migratory_birds(&arr));
}
