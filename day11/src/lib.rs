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

    let mut it = password.iter().copied().enumerate();
    let (_, init) = it.next().unwrap();
    it.fold(init, |prev, (idx, cur)| {
        if prev == cur {
            let idx_mask = (1 << (idx - 1)) as u16;
            indices[(cur - b'a') as usize] |= idx_mask;
        }

        cur
    });

    for (i, &a) in indices.iter().enumerate() {
        if a == 0 {
            continue;
        }

        for &b in indices.iter().skip(i + 1) {
            if b == 0 {
                continue;
            }

            if b & !(a >> 1) != 0 {
                return true;
            }
        }
    }

    false
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
