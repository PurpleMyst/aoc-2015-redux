const HLF: u8 = b'f';
const TPL: u8 = b'l';
const INC: u8 = b'c';
const JMP: u8 = b'p';
const JIO: u8 = b'o';

fn collatz(mut n: usize) -> usize {
    let mut i = 0;

    while n != 1 {
        i += 1;
        if n & 1 == 0 {
            n >>= 1;
        } else {
            n *= 3;
            n += 1;
        }
    }

    i
}

fn interpret(a: usize, instructions: impl Iterator<Item = u8>) -> usize {
    instructions.fold(a, |a, instr| match instr {
        HLF => a / 2,
        TPL => a * 3,
        INC => a + 1,
        _ => unreachable!(),
    })
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut instructions = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes()[2])
        .skip(1);

    let p1_a = interpret(0, instructions.by_ref().take_while(|&instr| instr != JMP));
    let p2_a = interpret(1, instructions.take_while(|&instr| instr != JIO));

    (collatz(p1_a), collatz(p2_a))
}
