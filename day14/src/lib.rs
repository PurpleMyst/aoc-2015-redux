const TIME: u16 = 2503;

// 0     1   2   3 0    1   2 0        1   2    3    4    5   6
// Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.

#[derive(Debug, Clone, Copy)]
pub struct Reindeer {
    speed: u8,
    flight_time: u8,
    rest_time: u8,

    resting: bool,
    counter: u8,
    traveled: u16,
    score: u16,
}

impl Reindeer {
    fn total_distance(&self) -> u16 {
        let time_block = u16::from(self.flight_time + self.rest_time);

        u16::from(self.speed)
            * (u16::from(self.flight_time) * (TIME / time_block)
                + (TIME % time_block).min(u16::from(self.flight_time)))
    }

    fn from_input(line: &str) -> Self {
        let mut words = line.split(' ');

        let speed = words.nth(3).unwrap().parse().unwrap();
        let flight_time = words.nth(2).unwrap().parse().unwrap();
        let rest_time = words.nth(6).unwrap().parse().unwrap();

        Reindeer {
            speed,
            flight_time,
            rest_time,

            resting: false,
            counter: flight_time,
            traveled: 0,
            score: 0,
        }
    }
}

#[inline]
pub fn solve_part1(reindeers: &[Reindeer]) -> u16 {
    reindeers.iter().map(|r| r.total_distance()).max().unwrap()
}

#[inline]
pub fn solve_part2(reindeers: &mut [Reindeer]) -> u16 {
    for _ in 0..TIME {
        let mut best_traveled = 0;

        for reindeer in reindeers.iter_mut() {
            reindeer.counter -= 1;

            if reindeer.resting {
                if reindeer.counter == 0 {
                    reindeer.resting = false;
                    reindeer.counter = reindeer.flight_time;
                }
            } else {
                reindeer.traveled += u16::from(reindeer.speed);

                if reindeer.counter == 0 {
                    reindeer.resting = true;
                    reindeer.counter = reindeer.rest_time;
                }
            }

            best_traveled = best_traveled.max(reindeer.traveled);
        }

        reindeers
            .iter_mut()
            .filter(|reindeer| reindeer.traveled == best_traveled)
            .for_each(|reindeer| reindeer.score += 1);
    }

    reindeers
        .iter()
        .map(|reindeer| reindeer.score)
        .max()
        .unwrap()
}

#[inline]
pub fn load_input() -> Vec<Reindeer> {
    include_str!("input.txt")
        .lines()
        .map(Reindeer::from_input)
        .collect()
}

#[inline]
pub fn solve() -> (u16, u16) {
    let mut reindeers = load_input();

    (solve_part1(&reindeers), solve_part2(&mut reindeers))
}
