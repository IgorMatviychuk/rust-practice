// 14 рішень
fn solve_cryptarithm() -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut solutions = Vec::new();
    
    for m in 1..=9 { 
        for u in 0..=9 {
            for x in 0..=9 {
                for a in 1..=9 { 
                    for s in 1..=9 {
                        for l in 0..=9 {
                            for o in 0..=9 {
                                for n in 0..=9 {
                                    let digits = vec![m, u, x, a, s, l, o, n];
                                    if digits.len() != digits.iter().collect::<std::collections::HashSet<_>>().len() {
                                        continue;
                                    }

                                    let left = (1000 * m + 100 * u + 10 * x + a) * a;
                                    let right = 1000 * s + 100 * l + 10 * o + n;
                                    
                                    if left == right {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                        println!("Знайдено рішення: m={}, u={}, x={}, a={}, s={}, l={}, o={}, n={}", m, u, x, a, s, l, o, n);
                                        println!("{} {} {} {}", m, u, x, a);
                                        println!("  x   {}", a);
                                        println!("--------");
                                        println!("  {} {} {} {}", s, l, o, n);
                                        println!();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
}

fn main() {
    let solutions = solve_cryptarithm();
    println!("Загальна кількість рішень: {}", solutions.len());
}
