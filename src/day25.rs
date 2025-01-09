fn sum_range(a: usize, b: usize) -> usize {
    if a == 1 {
        b * (b+1) / 2
    } else {
        sum_range(1, b) - sum_range(1, a - 1)
    }
}

// https://en.wikipedia.org/wiki/Modular_exponentiation
fn mod_exp(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

pub fn day25(input: &str) -> (String, String) {
    let (_, a) = input.split_once(" row ").unwrap();
    let (b, c) = a.split_once(", column ").unwrap();

    let start = 20151125;
    let (r,c): (usize, usize) = (b.parse().unwrap(), c.trim_end_matches(|x: char| !x.is_numeric()).parse().unwrap());

    let t: usize = sum_range(1,c);
    let n: usize = t + sum_range(c,c + r - 2);
    
    let base: usize = 252533;
    let mode: usize = 33554393;
    let foo = mod_exp(base, n - 1, mode);
    let code = (start * foo) % mode;

    let part1 = code;
    let part2 = 0;

    (format!("{part1}"), format!("{part2}"))
}
