use std::collections::HashSet;


/*fn __combine(containers: Vec<u8>, n: u8, cache: &mut HashMap<(Vec<u8>, u8), usize>) -> usize {
    if let Some(&value) = cache.get(&(containers.clone(), n)) {
        return value;
    }
    let mut count = 0;
    for (i, &size) in containers.iter().enumerate() {
        println!("{n}: container {size}");
        if size > n {
            continue
        } else if size == n {
            count += 1;
        } else {
            let mut subset = containers.clone();
            subset.remove(i);
            count += combine(subset, n - size, cache);
        }
    }
    cache.insert((containers, n), count);
    count
}*/

fn combine(containers: Vec<(u8, u8)>, n: u8) -> usize {
    let mut stack : Vec<HashSet<(u8,u8)>> = containers.iter().map(|&x| HashSet::from([x])).collect();
    let mut success: HashSet<Vec<(u8,u8)>> = HashSet::new();
    while let Some(combo) = stack.pop() {
        for &c in containers.iter() {
            let mut next_combo = combo.clone();
            if next_combo.insert(c) {
                let total: u8 = next_combo.iter().map(|x| x.1 ).sum();
                if total == n {
                    let mut good: Vec<(u8, u8)> = next_combo.iter().map(|x| *x).collect();
                    good.sort();
                    success.insert(good);
                } else if total < n {
                    stack.push(next_combo);
                }
            }
        }
    }
    println!("{:?}", success);
    success.len()
}

pub fn day17(input: &str) -> (String, String) {
    // let input = "20\n15\n10\n5\n5";
    let containers: Vec<(u8, u8)> = input.lines().enumerate()
        .map(|(i,x)| (i as u8, x.parse().unwrap()))
        .collect();

    let part1 = combine(containers, 150); //, &mut cache);

    // let part1 = 0;
    let part2 = 0;
    (format!("{part1}"), format!("{part2}"))
}