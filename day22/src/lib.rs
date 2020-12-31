// XXX: what happens if drain kills the boss?
use array_iterator::ArrayIterator;
use keyed_priority_queue::KeyedPriorityQueue;
use rustc_hash::FxHashMap as HashMap;

const STARTING_HP: u16 = 50;
const STARTING_MANA: u16 = 500;

const MAGIC_MISSILE: usize = 0;
const DRAIN: usize = 1;
const SHIELD: usize = 2;
const POISON: usize = 3;
const RECHARGE: usize = 4;

const SPELL_COSTS: [u16; 5] = [53, 73, 113, 173, 229];
const SPELL_DURATIONS: [u8; 5] = [0, 0, 6, 6, 5];

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct State {
    player_hp: u16,
    player_mana: u16,

    boss_hp: u16,
    effects: [u8; 5],
}

#[derive(Clone, Copy)]
struct Priority {
    f: u16,
    g: u16,
}

impl PartialEq for Priority {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Priority {}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f).reverse()
    }
}

enum NextState {
    SpellPlays([Option<State>; 5]),
    PoisonWin,
    Lose,
}

impl State {
    fn next(mut self, boss_dmg: u16, hard: bool) -> NextState {
        if hard {
            self.player_hp -= 1;

            if self.player_hp == 0 {
                return NextState::Lose;
            }
        }

        // apply poison effect
        let boss_hp = self
            .boss_hp
            .saturating_sub(if self.effects[POISON] == 0 { 0 } else { 3 });

        if boss_hp == 0 {
            return NextState::PoisonWin;
        }

        // apply recharge effect
        let player_mana = self.player_mana + if self.effects[RECHARGE] == 0 { 0 } else { 101 };

        // decrease effect timers
        let effects = {
            let mut effects = self.effects;
            effects
                .iter_mut()
                .for_each(|eff| *eff = eff.saturating_sub(1));
            effects
        };

        // states after the player's turn
        let mut states = [
            if player_mana >= SPELL_COSTS[MAGIC_MISSILE] {
                Some(State {
                    player_hp: self.player_hp,
                    player_mana: player_mana - SPELL_COSTS[MAGIC_MISSILE],
                    boss_hp: boss_hp.saturating_sub(4),
                    effects,
                })
            } else {
                None
            },
            if player_mana >= SPELL_COSTS[DRAIN] {
                Some(State {
                    player_hp: self.player_hp + 2,
                    player_mana: player_mana - SPELL_COSTS[DRAIN],
                    boss_hp: boss_hp.saturating_sub(2),
                    effects,
                })
            } else {
                None
            },
            if player_mana >= SPELL_COSTS[SHIELD] && effects[SHIELD] == 0 {
                let mut effects = effects;
                effects[SHIELD] = SPELL_DURATIONS[SHIELD];
                Some(State {
                    player_hp: self.player_hp,
                    player_mana: player_mana - SPELL_COSTS[SHIELD],
                    boss_hp,
                    effects,
                })
            } else {
                None
            },
            if player_mana >= SPELL_COSTS[POISON] && effects[POISON] == 0 {
                let mut effects = effects;
                effects[POISON] = SPELL_DURATIONS[POISON];
                Some(State {
                    player_hp: self.player_hp,
                    player_mana: player_mana - SPELL_COSTS[POISON],
                    boss_hp,
                    effects,
                })
            } else {
                None
            },
            if player_mana >= SPELL_COSTS[RECHARGE] && effects[RECHARGE] == 0 {
                let mut effects = effects;
                effects[RECHARGE] = SPELL_DURATIONS[RECHARGE];
                Some(State {
                    player_hp: self.player_hp,
                    player_mana: player_mana - SPELL_COSTS[RECHARGE],
                    boss_hp,
                    effects,
                })
            } else {
                None
            },
        ];

        // play out boss turn
        states.iter_mut().for_each(|maybe_state| {
            if let Some(state) = maybe_state {
                // apply shield effect
                let player_armor = if state.effects[SHIELD] == 0 { 0 } else { 7 };

                // apply poison effect
                state.boss_hp =
                    state
                        .boss_hp
                        .saturating_sub(if state.effects[POISON] == 0 { 0 } else { 3 });

                // apply recharge effect
                state.player_mana += if state.effects[RECHARGE] == 0 { 0 } else { 101 };

                // decrease effect timers
                state.effects = {
                    let mut effects = state.effects;
                    effects
                        .iter_mut()
                        .for_each(|eff| *eff = eff.saturating_sub(1));
                    effects
                };

                if state.boss_hp != 0 {
                    state.player_hp = state
                        .player_hp
                        .saturating_sub(boss_dmg.saturating_sub(player_armor).max(1));

                    if state.player_hp == 0 {
                        *maybe_state = None;
                    }
                }
            }
        });

        NextState::SpellPlays(states)
    }
}

pub fn play(boss_initial_hp: u16, boss_dmg: u16, hard: bool) -> u16 {
    let start = State {
        player_hp: STARTING_HP,
        player_mana: STARTING_MANA,

        boss_hp: boss_initial_hp,
        effects: [0; 5],
    };

    let mut open = KeyedPriorityQueue::new();
    open.push(start, Priority { f: 0, g: 0 });

    // g score is the cost of the cheapest (currently known) path from start to the given node
    let mut gs = HashMap::default();
    gs.insert(start, 0);

    while let Some((state, priority)) = open.pop() {
        if state.boss_hp == 0 {
            return priority.g;
        }

        match state.next(boss_dmg, hard) {
            NextState::Lose => {}

            NextState::PoisonWin => return priority.g,

            NextState::SpellPlays(neighbors) => {
                for (idx, neighbor) in ArrayIterator::new(neighbors)
                    .enumerate()
                    .filter_map(|(a, b)| Some((a, b?)))
                {
                    let tentative_g = priority.g + SPELL_COSTS[idx];

                    if gs.get(&neighbor).map_or(true, |&g| tentative_g < g) {
                        let f = tentative_g + neighbor.boss_hp;

                        gs.insert(neighbor, tentative_g);
                        open.push(neighbor, Priority { g: tentative_g, f });
                    }
                }
            }
        }
    }

    unreachable!()
}

#[inline]
pub fn load_input() -> (u16, u16) {
    let mut stats = include_str!("Input.txt")
        .lines()
        .map(|n| n.rsplitn(2, ' ').next().unwrap().parse().unwrap());

    let hp = stats.next().unwrap();
    let dmg = stats.next().unwrap();

    (hp, dmg)
}

#[inline]
pub fn solve_part1(boss_initial_hp: u16, boss_dmg: u16) -> u16 {
    play(boss_initial_hp, boss_dmg, false)
}

#[inline]
pub fn solve_part2(boss_initial_hp: u16, boss_dmg: u16) -> u16 {
    play(boss_initial_hp, boss_dmg, true)
}

#[inline]
pub fn solve() -> (u16, u16) {
    let (boss_initial_hp, boss_dmg) = load_input();
    (
        solve_part1(boss_initial_hp, boss_dmg),
        solve_part2(boss_initial_hp, boss_dmg),
    )
}
