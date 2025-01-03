use std::collections::HashMap;

type Key = (i32, i32);

fn deliver_gifts(houses: &mut HashMap<Key, u32>, input: &str, method: u32) {
    let mut first = method == 2;
    let mut pos = (0,0);
    houses.insert(pos, 1);

    for c in input.chars() {
        if method > 0 {
            first = !first;
            if first {
                continue
            }
        }
        match c {
            '^' => pos.0 -= 1,
            '>' => pos.1 += 1,
            'v' => pos.0 += 1,
            '<' => pos.1 -= 1,
            _ => ()
        }
        match houses.get(&pos) {
            Some(&number) => houses.insert(pos, number + 1),
            _ => houses.insert(pos, 1),
        };
    }
}

pub fn day03(input_lines: &str) -> (String, String) {
    let mut houses = HashMap::new();
    deliver_gifts(&mut houses, input_lines, 0);
    let part1 = format!("{}", houses.len());
    houses.clear();
    deliver_gifts(&mut houses, input_lines, 1);
    deliver_gifts(&mut houses, input_lines, 2);
    let part2 = format!("{}", houses.len());

    (part1, part2)
}