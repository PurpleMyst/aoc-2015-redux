#[allow(clippy::inconsistent_digit_grouping)]
const START: u64 = 2015_11_25;
const MULTIPLIER: u64 = 252_533;
const MODULUS: u64 = 33_554_393;

fn compute_multiplier(mut idx: u64) -> u64 {
    let mut base = MULTIPLIER;
    let mut result = 1;

    loop {
        if idx & 1 == 1 {
            result *= base;
            result %= MODULUS;
        }

        if idx == 1 {
            return result;
        }

        idx >>= 1;
        base *= base;
        base %= MODULUS;
    }
}

fn nth_triangular(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn compute_idx(row: u64, col: u64) -> u64 {
    nth_triangular(col) + (row - 1) * col + nth_triangular(row - 2) - 1
}

fn compute_code(idx: u64) -> u64 {
    (START * compute_multiplier(idx)) % MODULUS
}

#[inline]
pub fn solve() -> u64 {
    let mut parts = include_str!("input.txt")
        .trim()
        .rsplit(' ')
        .map(|n| &n[..n.len() - 1]);

    let col = parts.next().unwrap().parse().unwrap();
    parts.next().unwrap();
    let row = parts.next().unwrap().parse().unwrap();

    compute_code(compute_idx(row, col))
}
