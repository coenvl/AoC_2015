#[derive(Debug)]
struct Instruction<'a> {
    op: &'a str,
    arg: i16,
}

fn parse<'a>(input: &'a str) -> Vec<Instruction<'a>> {
    input
        .lines()
        .map(|line| {
            let (op, args) = line.split_once(" ").unwrap();
            if op == "jio" || op == "jie" {
                let (_, arg) = args.split_once(", ").unwrap();
                Instruction {
                    op,
                    arg: arg.parse().unwrap(),
                }
            } else if op == "jmp" {
                Instruction {
                    op,
                    arg: args.parse().unwrap(),
                }
            } else {
                Instruction {
                    op,
                    arg: (args == "b") as i16,
                }
            }
        })
        .collect()
}

fn run(instructions: &Vec<Instruction>, start: usize) -> usize {
    let mut registers: [usize; 2] = [start, 0]; // a = 0, b = 1
    let mut i: usize = 0;
    while i < instructions.len() {
        match instructions[i].op {
            "hlf" => registers[instructions[i].arg as usize] /= 2,
            "tpl" => registers[instructions[i].arg as usize] *= 3,
            "inc" => registers[instructions[i].arg as usize] += 1,
            "jmp" => {
                i = i.saturating_add_signed(instructions[i].arg as isize);
                continue;
            },
            "jie" => {
                i += if registers[0] % 2 == 0 {
                    instructions[i].arg as usize
                } else {
                    1
                };
                continue;
            }
            "jio" => {
                i += if registers[0] == 1 {
                    instructions[i].arg as usize
                } else {
                    1
                };
                continue;
            }
            _ => panic!("No such operation"),
        }
        i += 1;
    }
    registers[1]
}

pub fn day23(input: &str) -> (String, String) {
    let data: Vec<Instruction> = parse(input);

    let part1 = run(&data, 0);
    let part2 = run(&data, 1);

    (format!("{part1}"), format!("{part2}"))
}
