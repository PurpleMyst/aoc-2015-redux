const LUT: [&str; 4 * 4] = [
    "00", "01", "02", "03", "10", "11", "12", "13", "20", "21", "22", "23", "30", "31", "32", "33",
];

#[inline]
pub fn solve() -> (usize, usize) {
    let mut state = include_str!("input.txt").trim().to_string();
    let mut next_state = String::with_capacity(state.len() * 2);

    let mut part1 = 0;

    for i in 0..50 {
        next_state.clear();

        let mut it = state.chars();
        let first = it.next().unwrap();
        let (cnt, group) = it.fold((1, first), |(cnt, group), digit| {
            if digit == group {
                (cnt + 1, group)
            } else {
                next_state.push_str(LUT[4 * cnt + (group as u8 - b'0') as usize]);
                (1, digit)
            }
        });

        next_state.push_str(LUT[4 * cnt + (group as u8 - b'0') as usize]);

        state.clone_from(&next_state);

        if i == 39 {
            part1 = state.len();
        }
    }

    (part1, state.len())
}
