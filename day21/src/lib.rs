// const GOLD: u16 = 100;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Item {
    name: &'static str,
    cost: u16,
    dmg: u16,
    armor: u16,
}

const NOTHING: Item = Item {
    name: "nothing",
    cost: 0,
    dmg: 0,
    armor: 0,
};

macro_rules! section {
    ($n:ident = $($name:literal $cost:literal $dmg:literal $armor:literal)+) => {
        const $n: &[Item] = &[$(Item { name: $name, cost: $cost, dmg: $dmg, armor: $armor},)+];
    }
}

section!(WEAPONS =
    "Dagger"        8     4       0
    "Shortsword"   10     5       0
    "Warhammer"    25     6       0
    "Longsword"    40     7       0
    "Greataxe"     74     8       0
);

section!(ARMOR =
    "Leather"      13     0       1
    "Chainmail"    31     0       2
    "Splintmail"   53     0       3
    "Bandedmail"   75     0       4
    "Platemail"   102     0       5
);

section!(RINGS =
    "Damage +1"    25     1       0
    "Damage +2"    50     2       0
    "Damage +3"   100     3       0
    "Defense +1"   20     0       1
    "Defense +2"   40     0       2
    "Defense +3"   80     0       3
);

#[derive(Debug, Clone, Copy)]
pub struct Combatant {
    hp: u16,
    dmg: u16,
    armor: u16,
}

impl Combatant {
    fn attack(&self, other: &mut Self) {
        other.hp = other
            .hp
            .saturating_sub(self.dmg.saturating_sub(other.armor).max(1));
    }
}

pub fn load_input() -> Combatant {
    let mut stats = include_str!("Input.txt")
        .lines()
        .map(|n| n.rsplitn(2, ' ').next().unwrap().parse().unwrap());

    let hp = stats.next().unwrap();
    let dmg = stats.next().unwrap();
    let armor = stats.next().unwrap();
    Combatant { hp, dmg, armor }
}

fn player_wins(mut player: Combatant, mut boss: Combatant) -> bool {
    loop {
        player.attack(&mut boss);
        if boss.hp == 0 {
            return true;
        }

        boss.attack(&mut player);
        if player.hp == 0 {
            return false;
        }
    }
}

#[inline]
pub fn solve_part1(boss: Combatant) -> u16 {
    let mut p1 = u16::MAX;

    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring in RINGS {
                let spent = weapon.cost + armor.cost + ring.cost;

                let dmg = weapon.dmg + ring.dmg;
                let armor = armor.armor + ring.armor;

                let player = Combatant {
                    dmg,
                    armor,
                    hp: 100,
                };

                if player_wins(player, boss) {
                    p1 = p1.min(spent);
                }
            }
        }
    }

    p1
}

#[inline]
pub fn solve_part2(boss: Combatant) -> u16 {
    let mut p2 = 0;

    for weapon in WEAPONS {
        for armor in ARMOR.iter().chain(Some(&NOTHING)) {
            for (i, ring1) in RINGS.iter().enumerate() {
                for ring2 in RINGS.iter().skip(i + 1) {
                    let spent = weapon.cost + armor.cost + ring1.cost + ring2.cost;

                    let dmg = weapon.dmg + ring1.dmg + ring2.dmg;
                    let armor = armor.armor + ring1.armor + ring2.armor;

                    let player = Combatant {
                        dmg,
                        armor,
                        hp: 100,
                    };

                    if !player_wins(player, boss) {
                        p2 = p2.max(spent);
                    }
                }
            }
        }
    }

    p2
}

#[inline]
pub fn solve() -> (u16, u16) {
    let boss = load_input();

    (solve_part1(boss), solve_part2(boss))
}
