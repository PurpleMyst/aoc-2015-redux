const PART1_TARGET: usize = 40;
const PART2_TARGET: usize = 50;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut state = include_str!("input.txt")
        .trim()
        .bytes()
        .map(|b| b - b'0')
        .collect::<Vec<_>>();

    let mut next_state = Vec::with_capacity(state.len() * 2);

    let mut part1 = 0;

    for i in 0..PART2_TARGET {
        next_state.clear();
        state.reserve(state.len());
        next_state.reserve(state.len() * 2);

        let mut it = state.iter().copied();
        let first = it.next().unwrap();
        let (cnt, group) = it.fold((1, first), |(cnt, group), digit| {
            if digit == group {
                (cnt + 1, group)
            } else {
                next_state.push(cnt);
                next_state.push(group);
                (1, digit)
            }
        });

        next_state.push(cnt);
        next_state.push(group);

        state.clear();
        state.extend_from_slice(&next_state);

        if i == PART1_TARGET - 1 {
            part1 = state.len();
        }
    }

    (part1, state.len())
}
