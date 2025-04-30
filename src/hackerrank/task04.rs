//https://www.hackerrank.com/challenges/diagonal-difference?isFullScreen=true

fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];  
        secondary_diagonal_sum += arr[i][n - 1 - i];  
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()  
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); 
    let n: usize = input.trim().parse().unwrap();  
    
    let mut arr: Vec<Vec<i32>> = Vec::new();
    
    for _ in 0..n {
        let mut row_input = String::new();
        std::io::stdin().read_line(&mut row_input).unwrap();  
        let row: Vec<i32> = row_input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();  
        
        arr.push(row);
    }

    let result = diagonal_difference(arr);
    println!("{}", result);  
}
