use std::cmp::min;

fn surface(line: &str) -> (isize, isize) {
    let nums: Vec<isize> = line.split('x').map(|x| x.parse().unwrap()).collect();
    let a1: isize = nums[0] * nums[1];
    let a2 = nums[1] * nums[2];
    let a3 = nums[2] * nums[0];
    let p1: isize = 2 * (nums[0] + nums[1]);
    let p2 = 2 * (nums[1] + nums[2]);
    let p3 = 2 * (nums[2] + nums[0]);
    let v = nums[0] * nums[1] * nums[2];
    let paper = 2 * a1 + 2 * a2 + 2 * a3 + min(a1, min(a2, a3));
    let ribbon = v + min(p1, min(p2 , p3));
    return (paper, ribbon);
}

pub fn day02(input_lines: &str) -> (String, String) {
    // let lines: std::str::Lines<'_> = input_lines.lines();
    return input_lines.lines()
        .map(surface)
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .unwrap();
}
