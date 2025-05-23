//https://www.hackerrank.com/challenges/divisible-sum-pairs/problem?isFullScreen=true

use std::io;

fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i as usize] + ar[j as usize]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", divisible_sum_pairs(n, k, &ar));
}
