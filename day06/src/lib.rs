struct Coord {
    x: usize,
    y: usize,
}

const SIDE: usize = 1000;

fn parse_coord(s: &str) -> Coord {
    let mut it = s.splitn(2, ',').map(|n| n.parse().unwrap());
    let x = it.next().unwrap();
    let y = it.next().unwrap();
    Coord { x, y }
}

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    operation: Operation,
    from: Coord,
    to: Coord,
}

impl Instruction {
    fn from_input(line: &str) -> Self {
        let mut it = line.split_ascii_whitespace();

        let operation = match it.next().unwrap() {
            "turn" => match it.next().unwrap() {
                "on" => Operation::TurnOn,
                "off" => Operation::TurnOff,
                _ => unreachable!(),
            },

            "toggle" => Operation::Toggle,

            _ => unreachable!(),
        };

        let from = parse_coord(it.next().unwrap());
        it.next().unwrap();
        let to = parse_coord(it.next().unwrap());

        Self {
            operation,
            from,
            to,
        }
    }
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1 = vec![0u8; SIDE * SIDE].into_boxed_slice();
    let mut part2 = vec![0u8; SIDE * SIDE].into_boxed_slice();

    include_str!("input.txt")
        .lines()
        .map(Instruction::from_input)
        .for_each(|instr| match instr.operation {
            Operation::TurnOn => {
                for y in instr.from.y..=instr.to.y {
                    for x in instr.from.x..=instr.to.x {
                        part1[y * SIDE + x] = 1;
                        part2[y * SIDE + x] += 1;
                    }
                }
            }

            Operation::TurnOff => {
                for y in instr.from.y..=instr.to.y {
                    for x in instr.from.x..=instr.to.x {
                        part1[y * SIDE + x] = 0;
                        part2[y * SIDE + x] = part2[y * SIDE + x].saturating_sub(1);
                    }
                }
            }

            Operation::Toggle => {
                for y in instr.from.y..=instr.to.y {
                    for x in instr.from.x..=instr.to.x {
                        part1[y * SIDE + x] ^= 1;
                        part2[y * SIDE + x] += 2;
                    }
                }
            }
        });

    (
        part1.iter().map(|&n| n as usize).sum(),
        part2.iter().map(|&n| n as usize).sum(),
    )
}
