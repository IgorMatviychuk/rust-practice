use rand::{thread_rng, Rng};

pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1;
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut balance = 0i32;

    for &cargo in shipments {
        balance += cargo as i32 - avg as i32;
        moves += balance.abs() as usize;
    }

    moves as isize
}

pub fn count_permutation_result(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return Err("Equal distribution impossible");
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut balance = 0i32;

    for &cargo in shipments {
        balance += cargo as i32 - avg as i32;
        moves += balance.abs() as usize;
    }

    Ok(moves)
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    let avg = rng.gen_range(1..=20);
    let mut shipments = vec![avg; n];

    for _ in 0..n {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 0 {
            let delta = rng.gen_range(0..=shipments[i]);
            shipments[i] -= delta;
            shipments[j] += delta;
        }
    }

    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_example_1() {
        let shipments = vec![1, 1, 1, 1, 6];
        assert_eq!(count_permutation(&shipments), 4);
    }

    #[test]
    fn test_balanced_example_2() {
        let shipments = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&shipments), 4);
    }

    #[test]
    fn test_balanced_example_3() {
        let shipments = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&shipments), 7);
    }

    #[test]
    fn test_impossible_case() {
        let shipments = vec![1, 2, 3];
        assert_eq!(count_permutation(&shipments), -1);
    }

    #[test]
    fn test_generated_valid() {
        let v = gen_shipments(10);
        assert_ne!(count_permutation(&v), -1);
    }
}
