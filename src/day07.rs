use std::collections::HashMap;
use std::slice::Iter;

#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Set
}

type Instruction<'a> = (String, &'a Operator, String);

impl Operator {
    pub fn values() -> Iter<'static, Operator> {
        [Operator::And, Operator::Or, Operator::Lshift, Operator::Rshift, Operator::Not, Operator::Set].iter()
    }

    fn as_str(&self) -> &'static str {
        match self {
            Operator::And => " AND ",
            Operator::Or => " OR ",
            Operator::Lshift => " LSHIFT ",
            Operator::Rshift => " RSHIFT ",
            Operator::Not => "NOT ",
            Operator::Set => "",
        }
    }
}

fn parse(line: &str) -> (String, Instruction) {
    let foo: Vec<&str> = line.split(" -> ").collect();
    for op in Operator::values() {
        if line.find(op.as_str()).is_some() {
            let nums: Vec<&str> = foo[0].split(op.as_str()).collect();
            return match op {
                Operator::Set => (foo[1].to_owned(), (foo[0].to_owned(), op, String::new())),
                Operator::Not => (foo[1].to_owned(), (nums[1].to_owned(), op, String::new())),
                _ => (foo[1].to_owned(), (nums[0].to_owned(), op, nums[1].to_owned()))
            }
        }
    }
    panic!("Invalid line?");
}

fn wire(target: String, instructions: &HashMap<String, Instruction>, cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(&result) = cache.get(&target) {
        return result;
    }

    let result = match target.parse::<u16>() {
        Ok(value) => value,
        Err(_) => {
            let i: Instruction = instructions.get(&target).expect("Key not found").clone();
            match i.1 {
                    Operator::And => wire(i.0, instructions, cache) & wire(i.2, instructions, cache),
                    Operator::Or => wire(i.0, instructions, cache) | wire(i.2, instructions, cache),
                    Operator::Lshift => wire(i.0, instructions, cache) << wire(i.2, instructions, cache),
                    Operator::Rshift => wire(i.0, instructions, cache) >> wire(i.2, instructions, cache),
                    Operator::Not => ! wire(i.0, instructions, cache),
                    Operator::Set => wire(i.0, instructions, cache),
                }
            }
        };
    cache.insert(target, result);
    result
}

pub fn day07(input_lines: &str) -> (String, String) {
    let instr: HashMap<String, Instruction> = input_lines.lines().map(parse).collect();

    let mut cache = HashMap::new();
    let part1 = wire(String::from('a'), &instr, &mut cache);
    
    cache.clear();
    cache.insert("b".to_owned(), part1);
    
    let part2 = wire(String::from('a'), &instr, &mut cache);

    return (part1.to_string(), part2.to_string());
}


#[cfg(test)]
mod test {

use super::*;

#[test]
fn test_rshift() {
    assert_eq!(("jq".to_owned(), ("jp".to_owned(), &Operator::Rshift, "2".to_owned())), parse("jp RSHIFT 2 -> jq"));
}

#[test]
fn test_set() {
    assert_eq!(("a".to_owned(), ("lx".to_owned(), &Operator::Set, "".to_owned())), parse("lx -> a"));
}

#[test]
fn test_not() {
    assert_eq!(("ho".to_owned(), ("hn".to_owned(), &Operator::Not, "".to_owned())), parse("NOT hn -> ho"));
}

#[test]
fn test_input() {
    println!("{:?}", day07("123 -> x
456 -> y
x AND y -> d
x OR y -> a
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"));

}

}