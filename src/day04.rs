use md5;

pub fn day04(input_lines: &str) -> (String, String) {
    let mut d1: usize = 0;
    let mut d2: usize = 0;
    for i in 0..usize::MAX {
        let input = input_lines.to_owned() + &i.to_string();
        let digest = md5::compute(input);
        if d1 == 0 && digest.0[0..2] == [0,0] && digest.0[2] < 16 {
            d1 = i;
        }
        if d2 == 0 && digest.0[0..3] == [0,0,0] {
            d2 = i;
            break;
        }
    }
    return (d1.to_string(),d2.to_string());
}