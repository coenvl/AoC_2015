use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

fn calibrate(input: &str, replacements: &HashMap<&str, Vec<&str>>) -> usize {
    replacements.iter().flat_map(|(&k, &ref v)| {
        v.iter().flat_map(move |r| {
            input.match_indices(k).map(move |x| {
                format!("{}{}{}", &input[0..x.0], r, &input[x.0 + k.len()..])
            })
        })
    }).unique().count()
}

fn build(input: &str, replacements: &HashMap<&str, Vec<&str>>) -> Option<usize> {
    let reverse_map: HashMap<&str, &str> = replacements
        .iter()
        .flat_map(|(&k, v)| v.iter().map(move |&p| (p, k)))
        .collect();
    let keys: Vec<&str> = reverse_map
        .keys()
        .copied()
        .sorted_by(|&a, &b| b.len().cmp(&a.len()))
        .collect();

    let mut heap = BinaryHeap::from([(0, 0, input.to_owned())]);
    while let Some((len, n, molecule)) = heap.pop() {
        if molecule == "e" {
            return Some(n);
        }
        for k in keys.iter() {
            if molecule.contains(k) {
                let r = reverse_map.get(k).unwrap();
                let new_molecule = molecule.replacen(k, r, 1);
                heap.push((len + k.len() - r.len(), n + 1, new_molecule));
            }
        }
    }
    None
}

pub fn day19(input: &str) -> (String, String) {
    let (recipes, medicine) = input.split_once("\n\n").unwrap();

    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    recipes.lines().for_each(|line| {
        let (k, v) = line.split_once(" => ").unwrap();
        replacements.entry(k).or_insert(Vec::new()).push(v);
    });

    let part1 = calibrate(medicine.trim(), &replacements);
    let part2 = build(medicine.trim(), &replacements).expect("No solution found!");

    (format!("{part1}"), format!("{part2}"))
}
