//https://www.hackerrank.com/challenges/bon-appetit/problem?isFullScreen=true

use std::io;

fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let mut total = 0;
    for i in 0..bill.len() {
        if i as i32 != k {
            total += bill[i];
        }
    }
    
    let actual_share = total / 2;
    
    if actual_share == b {
        println!("Bon Appetit");
    } else {
        println!("{}", b - actual_share);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let bill: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    bon_appetit(&bill, k, b);
}
