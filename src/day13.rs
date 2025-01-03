use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse(txt: &str) -> HashMap<(&str, &str), i32> {
    txt.lines()
        .map(|x| {
            let words: Vec<&str> = x.split(' ').collect();
            let sign: i32 = if words[2] == "lose" { -1 } else { 1 };
            (
                (words[0], &words[10][0..words[10].len() - 1]),
                sign * words[3].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn cost(seating: &Vec<&&str>, costs: &HashMap<(&str, &str), i32>) -> i32 {
    seating
        .iter()
        .zip(seating.iter().cycle().skip(1))
        .map(|(&a, &b)| 
            costs.get(&(a, b)).unwrap_or(&0) + costs.get(&(b, a)).unwrap_or(&0)
        )
        .sum()
}

pub fn day13(input: &str) -> (String, String) {
    let edges: HashMap<(&str, &str), i32> = parse(input);
    let mut people: HashSet<&str> = edges.keys().map(|x| x.0).collect();

    let part1 = people
        .iter()
        .permutations(people.len())
        .map(|x| cost(&x, &edges))
        .max()
        .expect("No min found.");

    people.insert("");
    let part2 = people
        .iter()
        .permutations(people.len())
        .map(|x| cost(&x, &edges))
        .max()
        .expect("No min found.");
    (format!("{part1}"), format!("{part2}"))
}
