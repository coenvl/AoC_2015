use regex::Regex;

fn transform(line: &str) -> usize {
    let re: Regex = Regex::new(r"\\x[A-Fa-f0-9]{2}").unwrap();

    let len = line.len();
    // assert_eq!(line.chars().nth(0).unwrap(), '"');
    // assert_eq!(line.chars().nth(len-1).unwrap(), '"');
    let content = &line[1..len-1];
    let content = content.replace("\\\"","\"");
    let content = content.replace("\\\\","\\");
    let content = re.replace_all(content.as_str(), "?");
    return len - content.len();
}

fn encode(line: &str) -> usize {
    let len = line.len();
    let content = line.replace("\\","\\\\");
    let content = content.replace("\"","\\\"");
    return (content.len() + 2) - len;
}

pub fn day08(input_lines: &str) -> (String, String) {
    let part1: usize = input_lines.lines().map(transform).sum();
    let part2: usize = input_lines.lines().map(encode).sum();

    return (part1.to_string(), part2.to_string());
}


#[cfg(test)]
mod test {

use super::*;

#[test]
fn test_transform() {
    assert_eq!(transform("\"\""), 2);
    assert_eq!(transform("\"abc\""), 2);
    assert_eq!(transform("\"aaa\\\"aaa\""), 3);
    assert_eq!(transform("\"\\x27\""), 5);
}

#[test]
fn test_encode() {
    assert_eq!(encode("\"\""), 4);
    assert_eq!(encode("\"abc\""), 4);
    assert_eq!(encode("\"aaa\\\"aaa\""), 6);
    assert_eq!(encode("\"\\x27\""), 5)
}


}