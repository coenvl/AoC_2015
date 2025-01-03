fn is_nice(line: &&str) -> bool {
    let p1: usize = "aeiou".chars().map(|c| line.matches(c).count()).sum();
    let p2 = line.chars().zip(line.chars().skip(1)).find(|(a, b)| a == b);
    let p3 = ["ab", "cd", "pq", "xy"].iter().find(|&&x| line.contains(x));
    return p1 >= 3 && p2.is_some() && p3.is_none();
}

fn is_nice2(line: &&str) -> bool {
    let p1: usize = (0..line.len() - 2)
        .map(|i| line[i + 2..].matches(&line[i..i + 2]).count())
        .sum();
    let p2 = line.chars().zip(line.chars().skip(2)).find(|(a, b)| a == b);
    return p1 > 0 && p2.is_some();
}

pub fn day05(input_lines: &str) -> (String, String) {
    let part1 = input_lines.lines().filter(is_nice).count();
    let part2 = input_lines.lines().filter(is_nice2).count();

    return (part1.to_string(), part2.to_string());
}

#[cfg(test)]
mod test {

    use super::is_nice2;

    #[test]
    fn test1() {
        assert!(is_nice2(&"qjhvhtzxzqqjkmpb"))
    }

    #[test]
    fn test2() {
        assert!(is_nice2(&"xxyxx"))
    }

    #[test]
    fn test3() {
        assert!(!is_nice2(&"uurcxstgmygtbstg"))
    }

    #[test]
    fn test4() {
        assert!(!is_nice2(&"ieodomkazucvgmuy"))
    }

    #[test]
    fn test5() {
        assert!(!is_nice2(&"aaa"))
    }
}
