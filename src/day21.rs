#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Player {
    cost: u16,
    hp: u16,
    dmg: u16,
    arm: u16,
}

// type Player = (u16, u16, u16);
type Item = (u16, u16, u16);

fn parse(data: &str) -> Player {
    let stats: Vec<u16> = data
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    Player { cost: 0, hp: stats[0], dmg: stats[1], arm: stats[2]}
}


const WEAPONS: [Item; 5] = [(8,4,0),(10,5,0),(25,6,0),(40,7,0),(74,8,0)];
const ARMOR: [Item; 6] = [(0,0,0),(13,0,1),(31,0,2),(53,0,3),(75,0,4),(102,0,5)];
const RINGS: [Item; 7] = [(0,0,0),(25,1,0),(50,2,0),(100,3,0),(20,0,1),(40,0,2),(80,0,3)];
const WIN: bool = true;
const LOSE: bool = false;

pub fn day21(input: &str) -> (String, String) {
    let boss = parse(input);
    let mut players: Vec<Player> = WEAPONS.into_iter().flat_map(|weapon| {
        ARMOR.iter().flat_map(move |&armor| {
            RINGS.iter().flat_map(move |&ring1| {
                RINGS.iter().filter(move |&&ring2| {
                    (ring1 == (0,0,0) && ring2 == (0,0,0)) || ring2 != ring1
                }).map(move |ring2| {
                    Player {
                        cost: weapon.0 + armor.0 + ring1.0 + ring2.0,
                        hp: 100_u16, 
                        dmg: weapon.1 + ring1.1 + ring2.1,
                        arm: armor.2 + ring1.2 + ring2.2
                    }
                })
            })
        })
    }).collect();

    let outcomes: Vec<_> = players.iter_mut().filter_map(|player| {
        let mut boss2 = boss.clone();
        loop {
            boss2.hp = boss2.hp.saturating_sub(1.max(player.dmg.saturating_sub(boss2.arm)));
            if boss2.hp == 0 {
                return Some((WIN, player.cost));
            }
            player.hp = player.hp.saturating_sub(1.max(boss2.dmg.saturating_sub(player.arm)));
            if player.hp == 0 {
                return Some((LOSE, player.cost));
            }
        }
    }).collect();
    
    let part1 = outcomes.iter().filter_map(|x| if x.0 == WIN {Some(x.1)} else {None}).min().expect("No solution found");
    let part2 = outcomes.iter().filter_map(|x| if x.0 == LOSE {Some(x.1)} else {None}).max().expect("No solution found");

    (format!("{part1}"), format!("{part2}"))
}
