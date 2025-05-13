use rand::Rng;


pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1], min_sum)
}

pub fn print_vector_with_min_pair(data: &[i32]) {
    let (i, a, b, sum) = min_adjacent_sum(data);

    print!("indexes:");
    for idx in 0..data.len() {
        print!("{:>4}.", idx);
    }
    println!();

    print!("data:   ");
    for &val in data {
        print!("{:>4},", val);
    }
    println!();

    print!("indexes:{:>width$}\\__ __/", "", width = 5 * i);
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a, b, sum, i, i + 1
    );
}

/*mod homeworks;

fn main() {
    let data = homeworks::gen_random_vector(20);
    homeworks::print_vector_with_min_pair(&data);
}/*
