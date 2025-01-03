pub fn day06(input_lines: &str) -> (String, String) {
    let mut grid: [[bool; 1000]; 1000] = [[false; 1_000]; 1000];
    let mut grid2: [[u8; 1000]; 1000] = [[0; 1_000]; 1000];
    
    for line in input_lines.lines() {
        let nums: Vec<usize> = line
            .split(|c: char| !c.is_digit(10))
            .filter_map(|x| x.parse().ok())
            .collect();
        match &line[..7] {
            "turn on" => { // "turn on"
                for i in nums[0]..=nums[2] {
                    grid[i][nums[1]..=nums[3]].fill(true);
                    grid2[i][nums[1]..=nums[3]].iter_mut().map(|x| *x += 1).last();
                }
            }
            "turn of" => { // "turn off"
                for i in nums[0]..=nums[2] {
                    grid[i][nums[1]..=nums[3]].fill(false);
                    grid2[i][nums[1]..=nums[3]].iter_mut().map(|x| *x = x.saturating_sub(1)).last();
                }
            }
            "toggle " => {
                for i in nums[0]..=nums[2] {
                    grid[i][nums[1]..=nums[3]].iter_mut().map(|x| *x = !*x).last();
                    grid2[i][nums[1]..=nums[3]].iter_mut().map(|x| *x += 2).last();
                }
            }
            _ => {}
        }
    };

    let part1 = grid
        .iter()
        .map(|&x| x.iter().filter(|&&x| x).count()).sum::<usize>();

    let part2: usize = grid2.iter().map(|x| x.iter().map(|&x| x as usize).sum::<usize>()).sum();

    return (part1.to_string(), part2.to_string());
}


#[cfg(test)]
mod test {

use super::day06;

#[test]
fn test1() {
    assert_eq!(("1000000".to_string(),"0".to_string()), day06("turn on 0,0 through 999,999"));
}

}