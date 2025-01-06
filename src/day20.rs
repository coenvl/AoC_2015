use std::collections::HashSet;
use memoize::memoize;

const TARGET: u32 = 34000000;

#[memoize]
fn divisors(n: u32) -> HashSet<u32> {
    for &i in &[11,7,5,3,2] {
        if n % i == 0 {
            return divisors(n / i).into_iter().flat_map(|x| [x, n/x]).collect();
        }
    }
    let max = (n as f32).sqrt().floor() as u32 + 1;
    (1..max).filter(|x| n % x == 0).flat_map(|x| [x, n/x]).collect()
}

fn compute(target: u32) -> (Option<u32>, Option<u32>) {
    let mut part1 = None;
    let mut part2 = None;
    for i in 1..1000_000 {
        let d = divisors(i);
        
        let gifts_part1: u32 = 10 * d.iter().sum::<u32>();
        if gifts_part1 >= target && part1.is_none() {
            part1 = Some(i);
        }
        
        let gifts_part2 = 11 * d.iter().filter(|&d| i/d < 50).sum::<u32>();
        if gifts_part2 >= target && part2.is_none() {
            part2 = Some(i);
        }
        if part1.is_some() && part2.is_some() {
            break;
        }
    }
    (part1, part2)
}

pub fn day20(_input: &str) -> (String, String) { 
    let (part1,part2) = compute(TARGET);
    (format!("{}", part1.expect("No solution found")), format!("{}", part2.expect("No solution found")))
}
