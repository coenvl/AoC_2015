use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
struct State<'a> {
    visited: Vec<&'a String>,
    location: &'a String,
    distance: usize,
}

fn find_shortest_route(distances: &HashMap<(&String, &String), &usize>) -> HashSet<usize> {
    let locations: HashSet<&String> = distances.iter().flat_map(|x| [x.0 .0, x.0.1]).collect();
    let mut lengths: HashSet<usize> = HashSet::new();

    let mut queue: Vec<State> = Vec::new();
    for &start in locations.iter() {
        queue.push(State {
            visited: Vec::from([start]),
            location: start,
            distance: 0,
        });
    }

    while let Some(state) = queue.pop() {
        for &next_hop in locations.iter() {
            if !state.visited.contains(&next_hop) {
                let mut new_visited = state.visited.clone();
                new_visited.push(next_hop);
                
                let hop_distance: usize = **distances
                        .get(&(state.location, next_hop))
                        .or_else(|| distances.get(&(next_hop, state.location)))
                        .expect("Unable to find distance");
                let new_distance = state.distance + hop_distance;
                
                if new_visited.len() == locations.len() {
                    lengths.insert(new_distance);
                } else {
                    queue.push(State {
                        visited: new_visited,
                        location: next_hop,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    lengths
}

pub fn day09(input_lines: &str) -> (String, String) {
    let distances: HashMap<(String, String), usize> = input_lines
        .lines()
        .map(|x| {
            let a: Vec<&str> = x.split(" = ").collect();
            let b: Vec<&str> = a[0].split(" to ").collect();
            ((b[0].to_owned(), b[1].to_owned()),
                a[1].parse::<usize>().unwrap())
        })
        .collect();

    let lut: HashMap<(&String, &String), &usize> = distances
        .iter()
        .map(|x| ((&x.0 .0, &x.0 .1), x.1))
        .collect();
    let part1 = find_shortest_route(&lut);

    (format!("{:?}", part1.iter().min().unwrap()), format!("{}", part1.iter().max().unwrap()))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_day09() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let output = day09(input);
        println!("{output:?}");
    }
}
