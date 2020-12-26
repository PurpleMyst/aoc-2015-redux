use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum WireArg {
    Constant(u16),
    Name(&'static str),
}

impl From<&'static str> for WireArg {
    fn from(s: &'static str) -> Self {
        s.parse().map(WireArg::Constant).unwrap_or(WireArg::Name(s))
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Wire {
    Assign(WireArg),
    Not(&'static str),
    And(&'static str, &'static str),
    Or(&'static str, &'static str),
    LShift(&'static str, u16),
    RShift(&'static str, u16),
}

impl Wire {
    fn from_input(line: &'static str) -> (&'static str, Self) {
        let (value, name) = {
            let mut parts = line.split(" -> ");
            (parts.next().unwrap(), parts.next().unwrap())
        };

        (
            name,
            if value.starts_with("NOT") {
                Wire::Not(value.split_ascii_whitespace().nth(1).unwrap())
            } else if !value.contains(|c: char| c.is_ascii_whitespace()) {
                Wire::Assign(value.into())
            } else {
                let (lhs, op, rhs) = {
                    let mut parts = line.split_ascii_whitespace();
                    (
                        parts.next().unwrap(),
                        parts.next().unwrap(),
                        parts.next().unwrap(),
                    )
                };

                match op {
                    "OR" => Wire::Or(lhs, rhs),
                    "AND" => Wire::And(lhs, rhs),
                    "RSHIFT" => Wire::RShift(lhs, rhs.parse().unwrap()),
                    "LSHIFT" => Wire::LShift(lhs, rhs.parse().unwrap()),
                    _ => panic!("unknown operator {:?}", op),
                }
            },
        )
    }
}

struct Circuit {
    wires: HashMap<&'static str, Wire>,
    value_cache: HashMap<&'static str, u16>,
}

impl Circuit {
    fn new(wires: HashMap<&'static str, Wire>) -> Self {
        Self {
            value_cache: HashMap::with_capacity_and_hasher(wires.len(), Default::default()),
            wires,
        }
    }

    fn evaluate(&mut self, wire: impl Into<WireArg>) -> u16 {
        match wire.into() {
            WireArg::Constant(n) => n,
            WireArg::Name(wire) => {
                if let Some(value) = self.value_cache.get(wire) {
                    *value
                } else {
                    let value = match self.wires[wire] {
                        Wire::Assign(name) => self.evaluate(name),
                        Wire::Not(name) => !self.evaluate(name),
                        Wire::And(a, b) => self.evaluate(a) & self.evaluate(b),
                        Wire::Or(a, b) => self.evaluate(a) | self.evaluate(b),
                        Wire::LShift(a, b) => self.evaluate(a) << b,
                        Wire::RShift(a, b) => self.evaluate(a) >> b,
                    };

                    self.value_cache.insert(wire, value);
                    value
                }
            }
        }
    }
}

#[inline]
pub fn solve() -> (u16, u16) {
    let mut evaluator = Circuit::new(
        include_str!("input.txt")
            .lines()
            .map(Wire::from_input)
            .collect::<HashMap<_, _>>(),
    );

    let part1 = evaluator.evaluate("a");

    evaluator.value_cache.clear();
    evaluator
        .wires
        .insert("b", Wire::Assign(WireArg::Constant(part1)));
    let part2 = evaluator.evaluate("a");

    (part1, part2)
}
