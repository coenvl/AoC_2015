fn expand(pats: String) -> String {
    let mut i: usize = 0;
    let mut ret: String = String::new();
    let pat = pats.as_bytes();
    while i < pat.len() {
        if i < pat.len() - 2 && pat[i] == pat[i+1] && pat[i] == pat[i+2] {
            ret.push('3');
            i += 3;
        } else if i < pat.len() - 1 && pat[i] == pat[i+1] {
            ret.push('2');
            i += 2;
        } else {
            ret.push('1');
            i += 1;
        }
        ret.push(char::from(pat[i-1]));
    }
    return ret;
}

pub fn day10(input: &str) -> (String, String) {
    let mut txt = expand(input.to_string());
    let mut part1: usize = 0;
    for i in 0..49 {
      if i == 39 {
        part1 = txt.len();
      } 
      txt = expand(txt);
    }
    let part2 = txt.len();
    (format!("{part1}"), format!("{part2}"))
}