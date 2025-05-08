fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    let n = if n < 0 {
        len as isize + n % len as isize
    } else {
        n % len as isize
    };

    let n = n as usize;
    let (left, right) = s.split_at(len - n);
    right.to_string() + left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)| {
                assert_eq!(
                    rotate(s.clone(), *n),
                    exp.to_string()
                )
            });
    }
}
