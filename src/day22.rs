use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialOrd, Ord, Clone, Copy)]
struct Effect {
    timer: Option<u8>,
    armor: u8,
    damage: u8,
    hp: u8,
    mana: u16,
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        self.armor == other.armor
            && self.damage == other.damage
            && self.hp == other.hp
            && self.mana == other.mana
    }
}

impl Eq for Effect {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    total_mana: u16,
    hp: u8,
    mana: u16,
    boss_hp: u8,
    boss_dmg: u8,
    effects: Vec<Effect>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_mana.cmp(&other.total_mana).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(boss: (u8, u8)) -> State {
        return State {
            total_mana: 0,
            hp: 50,
            mana: 500,
            boss_hp: boss.0,
            boss_dmg: boss.1,
            effects: Vec::new(),
        };
    }

    fn apply_effects(self: &mut Self) -> u8 {
        let mut boss_damage = self.boss_dmg.clone();
        for e in self.effects.iter_mut() {
            self.boss_hp = self.boss_hp.saturating_sub(e.damage);
            self.mana += e.mana;
            self.hp += e.hp;
            e.timer = Some(e.timer.unwrap() - 1);
            boss_damage = boss_damage.saturating_sub(e.armor);
        }
        self.effects.retain(|e| e.timer.unwrap() > 0);
        boss_damage
    }
}

fn parse(data: &str) -> (u8, u8) {
    let stats: Vec<u8> = data
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    (stats[0], stats[1])
}

fn player_turn(state: &mut State, cost: u16, effect: &Effect) {
    state.apply_effects();
    state.mana -= cost;
    state.total_mana += cost;
    
    if effect.timer.is_none() {
        state.boss_hp = state.boss_hp.saturating_sub(effect.damage);
        state.hp += effect.hp;
    } else {
        state.effects.push(effect.clone());
    }
}

fn boss_turn(state: &mut State) {
    let boss_damage = state.apply_effects();
    state.hp = state.hp.saturating_sub(boss_damage)
}

fn find_best(init: &State, shop: &HashMap<u16, Effect>, hard_mode: bool) -> Option<u16> {
    let mut heap: BinaryHeap<State> = BinaryHeap::from([init.clone()]);
    while let Some(state) = heap.pop() {
        for (&cost, effect) in shop.iter() {
            if state.mana < cost
                || state
                    .effects
                    .iter()
                    .any(|e| e == effect && e.timer.unwrap() > 1)
            {
                continue;
            }

            let mut new_state = state.clone();
            if hard_mode {
                new_state.hp -= 1;
                if new_state.hp == 0 {
                    continue;
                }
            }

            // Player turn
            player_turn(&mut new_state, cost, effect);
            if new_state.boss_hp == 0 {
                return Some(new_state.total_mana);
            }

            // Boss turn
            boss_turn(&mut new_state);
            if new_state.boss_hp == 0 {
                return Some(new_state.total_mana);
            } else if new_state.hp == 0 {
                continue;
            } else {
                heap.push(new_state);
            }
        }
    }
    None
}

pub fn day22(input: &str) -> (String, String) {
    let boss = parse(input);

    let init: State = State::new(boss);
    let mut shop: HashMap<u16, Effect> = HashMap::new();
    shop.insert(
        53,
        Effect {
            timer: None,
            armor: 0,
            damage: 4,
            hp: 0,
            mana: 0,
        },
    );
    shop.insert(
        73,
        Effect {
            timer: None,
            armor: 0,
            damage: 2,
            hp: 2,
            mana: 0,
        },
    );
    shop.insert(
        113,
        Effect {
            timer: Some(6),
            armor: 7,
            damage: 0,
            hp: 0,
            mana: 0,
        },
    );
    shop.insert(
        173,
        Effect {
            timer: Some(6),
            armor: 0,
            damage: 3,
            hp: 0,
            mana: 0,
        },
    );
    shop.insert(
        229,
        Effect {
            timer: Some(5),
            armor: 0,
            damage: 0,
            hp: 0,
            mana: 101,
        },
    );

    let part1 = find_best(&init, &shop, false).expect("No solution found!"); // 900 is correct
    let part2 = find_best(&init, &shop, true).expect("No solution found!"); // 1216 is correct

    (format!("{part1}"), format!("{part2}"))
}
