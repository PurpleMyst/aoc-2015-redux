use std::borrow::Cow;

fn solve_part1(mut inp: &[u8]) -> i64 {
    let mut part1 = 0;

    while let Some(idx) = inp.iter().position(|&ch| matches!(ch, b'-' | b'0'..=b'9')) {
        let (_, inp2) = inp.split_at(idx);

        let (sign, mut inp2) = if let Some((&b'-', inp2)) = inp2.split_first() {
            (-1, inp2)
        } else {
            (1, inp2)
        };

        let mut res = 0;

        while let Some((ch, inp3)) = inp2.split_first() {
            if matches!(ch, b'0'..=b'9') {
                res = 10 * res + (ch - b'0') as i64;
            } else {
                break;
            }

            inp2 = inp3;
        }

        part1 += res * sign;

        inp = inp2;
    }

    part1
}

fn sum_up(value: simd_json::value::BorrowedValue) -> i64 {
    match value {
        simd_json::BorrowedValue::Static(node) => match node {
            simd_json::StaticNode::I64(n) => n,
            simd_json::StaticNode::U64(n) => n as i64,
            _ => 0,
        },

        simd_json::BorrowedValue::String(..) => 0,

        simd_json::BorrowedValue::Array(items) => items.into_iter().map(sum_up).sum(),

        simd_json::BorrowedValue::Object(objs) => {
            if objs
                .values()
                .any(|v| v == &simd_json::BorrowedValue::String(Cow::Borrowed("red")))
            {
                return 0;
            }

            objs.into_iter().map(|(_, v)| sum_up(v)).sum()
        }
    }
}

fn solve_part2(inp: &[u8]) -> i64 {
    let mut inp = Box::from(inp);
    let tree: simd_json::BorrowedValue = simd_json::deserialize(&mut inp).unwrap();
    sum_up(tree)
}

#[inline]
pub fn solve() -> (i64, i64) {
    let inp = include_str!("input.txt").trim().as_bytes();
    (solve_part1(inp), solve_part2(inp))
}
