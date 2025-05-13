// - https://www.hackerrank.com/challenges/birthday-cake-candles/problem?isFullScreen=true

use std::io;

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = candles.iter().max().unwrap_or(&0);
    candles.iter().filter(|&&h| h == *max_height).count() as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let candles: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let result = birthday_cake_candles(&candles);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_single_candle() {
        let candles = vec![1];
        assert_eq!(birthday_cake_candles(&candles), 1);
    }

    #[test]
    fn test_all_same_height() {
        let candles = vec![2, 2, 2, 2];
        assert_eq!(birthday_cake_candles(&candles), 4);
    }

    #[test]
    fn test_empty() {
        let candles: Vec<i32> = vec![];
        assert_eq!(birthday_cake_candles(&candles), 0);
    }
}
