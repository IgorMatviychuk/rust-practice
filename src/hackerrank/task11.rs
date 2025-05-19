//https://www.hackerrank.com/challenges/apple-and-orange/problem?isFullScreen=true

use std::io;

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {
    let apple_count = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    let orange_count = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let st: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let ab: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let apples: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let oranges: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    count_apples_and_oranges(st[0], st[1], ab[0], ab[1], apples, oranges);
}
