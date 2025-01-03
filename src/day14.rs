use std::collections::HashMap;

fn parse(txt: &str) -> Vec<(&str, usize, usize, usize)> {
    txt.lines().map(|x| {
        let words: Vec<&str> = x.split(' ').collect();
        (words[0],
         words[3].parse::<usize>().unwrap(),
         words[6].parse::<usize>().unwrap(),
         words[13].parse::<usize>().unwrap())
    }).collect()
}

fn travel((_name, v, t,r): &(&str, usize, usize, usize), i: usize) -> usize {
    let n = i / (t+r);
    let rem = i % (t+r);
    let t1 = rem.min(*t);
    (n * v * t) + (v * t1)
}

pub fn day14(input: &str) -> (String, String) {
    //let start = Instant::now();
    let data = parse(input);
    
    let part1: usize = data.iter()
      .map(|x| travel(x, 2503)).max().unwrap();

    let mut score: HashMap<&str, u16> = HashMap::new();
    (1..2503).map(|i| {
        let winner = data.iter().map(|x| {
            (travel(x, i), x.0)
        }).max().unwrap();
        *score.entry(winner.1).or_insert(0) += 1;
    }).last();
    let part2 = score.values().max().unwrap();
    
    (format!("{part1}"), format!("{part2}"))
}