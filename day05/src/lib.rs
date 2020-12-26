use std::convert::TryInto;

fn is_nice_part1(s: &[u8; 16]) -> bool {
    let mut vowels = 0;
    let mut contains_double = false;

    let mut it = s.iter().copied();
    let init = it.next().unwrap();

    if matches!(init, b'a' | b'e' | b'i' | b'o' | b'u') {
        vowels += 1;
    }

    it.try_fold(init, |prev, cur| {
        if prev == cur {
            contains_double = true;
        }

        if matches!(cur, b'a' | b'e' | b'i' | b'o' | b'u') {
            vowels += 1;
        }

        if matches!(
            (prev, cur),
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
        ) {
            return None;
        }

        Some(cur)
    })
    .is_some()
        && vowels >= 3
        && contains_double
}

fn is_nice_part2(s: &[u8; 16]) -> bool {
    // Second rule: just iterate with an offset
    if s.iter()
        .copied()
        .zip(s.iter().copied().skip(2))
        .find(|(a, b)| a == b)
        .is_none()
    {
        return false;
    }

    // First rule:
    // Build a lookup table where the index is the pair of chars as a u16 and
    // the value is a bitmask representing the indices at which we can find the pair
    let mut indices = [0; u16::from_be_bytes([b'z', b'z']) as usize + 1];

    let mut it = s.iter().copied().enumerate();
    let (_, init) = it.next().unwrap();
    it.try_fold(init, |prev, (idx, cur)| {
        // Build the index mask by bitshifting one to the left (idx - 1) times
        // We subtract one since the index we have corresponds to the last term in the pair
        let idx_mask = (1 << (idx - 1)) as u16;

        // Merge the two characters into one u16
        let pair = u16::from_be_bytes([prev, cur]) as usize;

        // Check if there's a version of this pair anywhere excluding the index before this one
        if indices[pair] & !(idx_mask >> 1) != 0 {
            return None;
        }

        // Add this pair to the lookup table
        indices[pair] |= idx_mask;

        Some(cur)
    })
    .is_none()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    include_str!("input.txt").lines().for_each(|s| {
        let s = s.as_bytes().try_into().unwrap();

        if is_nice_part1(&s) {
            part1 += 1;
        }

        if is_nice_part2(&s) {
            part2 += 1;
        }
    });

    (part1, part2)
}
