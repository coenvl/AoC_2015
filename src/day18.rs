const SIZE: usize = 100;
const MASK: [(i8,i8); 8] = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
const ON: u8 = b'#';
const OFF: u8 = b'.';

fn safe_get(field: &Vec<Vec<u8>>, i: i8, j: i8) -> Option<u8> {
    if i >= 0 && i < SIZE as i8 && j >= 0 && j < SIZE as i8  {
        Some(field[i as usize][j as usize])
    } else {
        None
    }
}

fn tick(field: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..field.len()).map(|i| {
        (0..field[i].len()).map(|j| {
            let neighbors: u8 = MASK.iter()
                .filter_map(|&n| safe_get(field, n.0 + i as i8, n.1 + j as i8))
                .filter(|&c| c == ON)
                .count() as u8;
            match neighbors {
                2 if field[i][j] == ON => ON,
                3 => ON,
                _ => OFF
            }
        }).collect()
    }).collect()
}

fn stuck(field: &mut Vec<Vec<u8>>) {
    field[0][0] = ON;
    field[SIZE-1][0] = ON;
    field[0][SIZE-1] = ON;
    field[SIZE-1][SIZE-1] = ON;
}

pub fn day18(input: &str) -> (String, String) {
    let mut data: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    for _ in 0..100 {
        data = tick(&data);
    }

    let part1: usize = data.iter().map(|line| line.iter().filter(|&&c| c == ON).count()).sum();

    let mut data: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    for _ in 0..100 {
        stuck(&mut data);
        data = tick(&data);
    }
    stuck(&mut data);
    let part2: usize = data.iter().map(|line| line.iter().filter(|&&c| c == ON).count()).sum();

    (format!("{part1}"), format!("{part2}"))
}
