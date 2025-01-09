use std::collections::HashMap;

fn combine(n: u8, list: &[u8], l: u8, lengths: &mut HashMap<u8, u16>) -> usize {
    if n == 0 {
        *lengths.entry(l).or_insert(0) += 1;
        1
    } else if list.len() == 0 {
        0
    } else {
        let p1 = combine(n, &list[1..], l, lengths);
        if list[0] <= n {
            p1 + combine(n - list[0], &list[1..], l+1, lengths)
        } else {
            p1
        }
    }
}

pub fn day17(input: &str) -> (String, String) {
    // let input = "20\n15\n10\n5\n5";
    let containers: Vec<u8> = input.lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut lengths: HashMap<u8, u16> = HashMap::new();
    let part1 = combine(150, &containers, 0, &mut lengths);
    let part2 = lengths.values().min().expect("No solution found");

    (format!("{part1}"), format!("{part2}"))
}