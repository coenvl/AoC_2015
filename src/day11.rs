fn fix(bytes: &mut [u8]) {
    for i in 0..bytes.len() {
        if bytes[i] == b'i' || bytes[i] == b'l' || bytes[i] == b'o' {
            bytes[i] += 1;
            for j in (i+1)..bytes.len(){
                bytes[j] = b'a';
            }
            return;
        }
    }
}

fn step(bytes: &mut [u8]) {
    for i in (0..bytes.len()).rev() {
        bytes[i] += 1;
        if bytes[i] == b'i' || bytes[i] == b'l' || bytes[i] == b'o' {
            bytes[i] += 1;
        }
        if bytes[i] > b'z' {
            bytes[i] = b'a';
        } else {
            return;
        }
    }
}

fn is_valid(bytes: &[u8]) -> bool {
    let mut straight = 0;
    let mut pair = 0;
    let mut pair_i: Option<usize> = None;
    for i in 0..(bytes.len()-1) {
        if bytes[i] == bytes[i+1] && (pair_i.is_none() || pair_i.unwrap() < i-1) {
          pair += 1;
          pair_i = Some(i);
        }
        if i < bytes.len()-2 && bytes[i] == bytes[i+1] - 1 && bytes[i+1] == bytes[i+2] - 1 {
          straight += 1;
        }
    }
    return pair > 1 && straight > 0;
}

fn find_next(pass: &mut str) {
    let bytes = unsafe { pass.as_bytes_mut() };
    fix(bytes);
    loop {
      step(bytes);
      if is_valid(bytes) {
          return;
      }
    }
}

pub fn day11(input: &str) -> (String, String) {
    let mut pass = String::from(input);
    
    find_next(& mut pass);
    let part1 = pass.clone();
    
    find_next(& mut pass);
    (format!("{part1}"), format!("{pass}"))
}
