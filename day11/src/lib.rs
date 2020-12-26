use std::convert::TryInto;

const PASSWORD_LEN: usize = 8;

fn is_valid(password: &[u8; PASSWORD_LEN]) -> bool {
    if !password
        .windows(3)
        .any(|win| win[0] + 1 == win[1] && win[1] + 1 == win[2])
    {
        return false;
    }

    if jetscii::bytes!(b'i', b'o', b'l')
        .find(&password[..])
        .is_some()
    {
        return false;
    }

    let mut indices = [0; (b'z' - b'a') as usize + 1];
    let mut set: u32 = 0;

    let mut it = password.iter().copied().enumerate();
    let (_, init) = it.next().unwrap();
    it.fold(init, |prev, (idx, cur)| {
        if prev == cur {
            let idx_mask = (1 << (idx - 1)) as u16;
            indices[(cur - b'a') as usize] |= idx_mask;
            set |= 1 << (cur - b'a');
        }

        cur
    });

    if set.count_ones() != 2 {
        debug_assert!(set.count_ones() == 0 || set.count_ones() == 1);
        return false;
    }

    let a = indices[set.leading_zeros() as usize - (32 - 26)];
    let b = indices[set.trailing_zeros() as usize];

    b & !(a >> 1) != 0
}

fn increment(password: &mut [u8; PASSWORD_LEN]) {
    for elem in password.iter_mut().rev() {
        if *elem == b'z' {
            *elem = b'a';
        } else {
            *elem += 1;
            break;
        }
    }
}

#[inline]
pub fn solve() -> ([u8; PASSWORD_LEN], [u8; PASSWORD_LEN]) {
    let mut password: [u8; PASSWORD_LEN] = include_str!("input.txt")
        .trim()
        .as_bytes()
        .try_into()
        .unwrap();

    while !is_valid(&password) {
        increment(&mut password);
    }
    let part1 = password;

    increment(&mut password);
    while !is_valid(&password) {
        increment(&mut password);
    }

    (part1, password)
}
