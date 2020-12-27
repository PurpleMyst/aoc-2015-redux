const CHILDREN: u8 = 3;
const CATS: u8 = 7;
const SAMOYEDS: u8 = 2;
const POMERANIANS: u8 = 3;
const AKITAS: u8 = 0;
const VIZSLAS: u8 = 0;
const GOLDFISH: u8 = 5;
const TREES: u8 = 3;
const CARS: u8 = 2;
const PERFUMES: u8 = 1;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1 = None;
    let mut part2 = None;

    for (idx, line) in include_str!("input.txt").lines().enumerate() {
        let compounds = line.splitn(2, ": ").nth(1).unwrap();

        let mut v1 = true;
        let mut v2 = true;

        for compound in compounds.split(", ") {
            if !v1 && !v2 {
                break;
            }

            let mut it = compound.splitn(2, ": ");
            let name = it.next().unwrap();
            let value: u8 = it.next().unwrap().parse().unwrap();

            if v1 {
                v1 = value
                    == match name {
                        "children" => CHILDREN,
                        "cats" => CATS,
                        "samoyeds" => SAMOYEDS,
                        "pomeranians" => POMERANIANS,
                        "akitas" => AKITAS,
                        "vizslas" => VIZSLAS,
                        "goldfish" => GOLDFISH,
                        "trees" => TREES,
                        "cars" => CARS,
                        "perfumes" => PERFUMES,
                        _ => unreachable!(),
                    };
            }

            if v2 {
                v2 = match name {
                    "children" => value == CHILDREN,
                    "cats" => value > CATS,
                    "samoyeds" => value == SAMOYEDS,
                    "pomeranians" => value < POMERANIANS,
                    "akitas" => value == AKITAS,
                    "vizslas" => value == VIZSLAS,
                    "goldfish" => value < GOLDFISH,
                    "trees" => value > TREES,
                    "cars" => value == CARS,
                    "perfumes" => value == PERFUMES,
                    _ => unreachable!(),
                };
            }
        }

        if part1.is_none() && v1 {
            part1 = Some(idx + 1);
        }

        if part2.is_none() && v2 {
            part2 = Some(idx + 1);
            break;
        }
    }

    (part1.unwrap(), part2.unwrap())
}
