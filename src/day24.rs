fn find_smallest_sets(data: &Vec<usize>, target: usize) -> Vec<Vec<usize>> {
    let mut ret: Vec<Vec<usize>> = Vec::new();
    let mut best_len = data.len();
    let mut stack: Vec<(Vec<usize>, usize)> = data.iter().map(|&x| (vec![x], x)).collect();
    while let Some((set, sum)) = stack.pop() {
        for e in data {
            if set.last().unwrap() <= e || sum + e > target || set.len() >= best_len {
                continue;
            }
            let mut new_set = set.clone();
            new_set.push(*e);
            if sum + e == target {
                best_len = best_len.min(new_set.len());
                ret.push(new_set);
            } else {
                stack.push((new_set, sum + e));
            }
        }
    }
    ret.into_iter().filter(|v| v.len() == best_len).collect()
}

pub fn day24(input: &str) -> (String, String) {
    let data: Vec<usize> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let sum: usize = data.iter().sum::<usize>();

    let sets = find_smallest_sets(&data, sum / 3);
    let qe: Vec<usize> = sets.iter().map(|v| v.iter().product()).collect();

    let part1 = qe.iter().min().expect("No solution found!");

    let sets2 = find_smallest_sets(&data, sum / 4);
    let qe2: Vec<usize> = sets2.iter().map(|v| v.iter().product()).collect();

    let part2 = qe2.iter().min().expect("No solution found!");

    (format!("{part1}"), format!("{part2}"))
}
