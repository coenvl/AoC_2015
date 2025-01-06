use std::collections::HashMap;

type Properties<'a> = HashMap<&'a str, u8>;

//Sue 2: akitas: 10, perfumes: 10, children: 5
fn parse(txt: &str) -> Vec<Properties> {
    txt.lines()
        .map(|line| {
            let mut iter = line.split_whitespace().skip(2);
            (0..3).map(|_| {
                (
                    iter.next().unwrap().trim_end_matches(':'),
                    iter.next().unwrap().trim_end_matches(',').parse().unwrap(),
                )
            })
            .collect()
        })
        .collect()
}

fn check(aunt: &Properties, filter: &Properties) -> bool {
    aunt.iter().all(|(k, v)| filter.get(k) == Some(v))
}

fn check2(aunt: &Properties, filter: &Properties) -> bool {
    aunt.iter().all(|(&k, v)| match k {
        "cats" | "trees" => Some(v) > filter.get(k),
        "pomeranians" | "goldfish" => Some(v) < filter.get(k),
        _ => filter.get(k) == Some(v),
    })
}

const MSG: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

pub fn day16(input: &str) -> (String, String) {
    let data = parse(input);
    let filter: Properties = MSG
        .lines()
        .map(|x| x.split_once(": ").unwrap())
        .map(|(p, n)| (p, n.parse().unwrap()))
        .collect();

    let aunt1 = data.iter().enumerate().find(|(_i, a)| check(a, &filter));

    let part1 = aunt1.expect("No match found!").0 + 1;

    let aunt2 = data.iter().enumerate().find(|(_i, a)| check2(a, &filter));
    let part2 = aunt2.expect("No match found!").0 + 1;

    (format!("{part1}"), format!("{part2}"))
}
