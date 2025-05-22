//https://www.hackerrank.com/challenges/between-two-sets/problem?isFullScreen=true

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn gcd_list(arr: &[i32]) -> i32 {
    arr.iter().fold(arr[0], |acc, &x| gcd(acc, x))
}

fn lcm_list(arr: &[i32]) -> i32 {
    arr.iter().fold(arr[0], |acc, &x| lcm(acc, x))
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let l = lcm_list(a);
    let g = gcd_list(b);

    let mut count = 0;
    let mut multiple = l;

    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

fn main() {
    use std::io::{self, BufRead};
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _nm: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|x| x.parse().unwrap()).collect();

    let a: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|x| x.parse().unwrap()).collect();

    let b: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", get_total_x(&a, &b));
}
