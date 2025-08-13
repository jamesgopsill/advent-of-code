use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/21.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(_: &str) -> String {
    let mut boss = Boss::new(8, 1, 104);

    let mut player = Player::new(Weapon::Shortsword, 100);
    let mut cost: u32 = 999_999;
    for weapon in Weapon::iter() {
        player.change_rings(vec![]);
        player.change_armor(Armor::None);
        player.change_weapon(weapon);

        for armor in Armor::iter() {
            player.change_armor(armor);

            // No rings
            let c = fight(&mut player, &mut boss);
            if c.is_some() {
                let c = c.unwrap();
                if c < cost {
                    cost = c;
                }
            }

            // Single rings
            for ring in Ring::iter() {
                player.change_rings(vec![ring]);

                let c = fight(&mut player, &mut boss);
                if c.is_some() {
                    let c = c.unwrap();
                    if c < cost {
                        cost = c;
                    }
                }
            }

            // combinations of rings
            for rings in Ring::iter().combinations(2) {
                player.change_rings(rings);

                let c = fight(&mut player, &mut boss);
                if c.is_some() {
                    let c = c.unwrap();
                    if c < cost {
                        cost = c;
                    }
                }
            }
        }
    }

    println!("Cheapest Setup: {cost}");

    /*
    let mut player = Player::new(Weapon::Shortsword, 8);
    player.change_armor(Armor::Platemail);

    loop {
        println!("{} {}", player.hp, boss.hp);
        player.attack(&mut boss);
        boss.attack(&mut player);
        if boss.hp == 0 {
            println!("Player Wins");
            break;
        }
        if player.hp == 0 {
            println!("Boss Wins");
            break;
        }
    }

    */

    0.to_string()
}

fn fight(p: &mut Player, b: &mut Boss) -> Option<u32> {
    p.hp = 100;
    b.hp = 104;
    loop {
        p.attack(b);
        b.attack(p);
        if b.hp == 0 {
            println!("Player Wins");
            return Some(p.cost());
        }
        if p.hp == 0 {
            println!("Boss Wins");
            return None;
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Weapon {
    Dagger,
    Shortsword,
    Warhammer,
    Longsword,
    Greataxe,
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Armor {
    None,
    Leather,
    Chainmail,
    Splintmail,
    Bandedmail,
    Platemail,
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Ring {
    Damage1,
    Damage2,
    Damage3,
    Defence1,
    Defence2,
    Defence3,
}

struct Player {
    weapon: Weapon,
    armor: Armor,
    rings: Vec<Ring>,
    hp: u32,
    damage: u32,
    defence: u32,
}

impl Player {
    fn new(weapon: Weapon, hp: u32) -> Self {
        let mut s = Self {
            weapon,
            armor: Armor::None,
            rings: vec![],
            hp,
            damage: 0,
            defence: 0,
        };
        s.update_damage_stat();
        s.update_defence_stat();
        s
    }

    fn cost(&self) -> u32 {
        let mut cost: u32 = 0;
        match self.weapon {
            Weapon::Dagger => cost += 8,
            Weapon::Shortsword => cost += 10,
            Weapon::Warhammer => cost += 25,
            Weapon::Longsword => cost += 40,
            Weapon::Greataxe => cost += 74,
        }
        match self.armor {
            Armor::None => {}
            Armor::Leather => cost += 13,
            Armor::Chainmail => cost += 31,
            Armor::Splintmail => cost += 53,
            Armor::Bandedmail => cost += 75,
            Armor::Platemail => cost += 102,
        }
        for ring in self.rings.iter() {
            match ring {
                Ring::Damage1 => cost += 25,
                Ring::Damage2 => cost += 50,
                Ring::Damage3 => cost += 100,
                Ring::Defence1 => cost += 20,
                Ring::Defence2 => cost += 40,
                Ring::Defence3 => cost += 80,
            }
        }
        cost
    }

    fn update_damage_stat(&mut self) {
        self.damage = 0;
        match self.weapon {
            Weapon::Dagger => self.damage += 4,
            Weapon::Shortsword => self.damage += 5,
            Weapon::Warhammer => self.damage += 6,
            Weapon::Longsword => self.damage += 7,
            Weapon::Greataxe => self.damage += 8,
        }
        for ring in self.rings.iter() {
            match ring {
                Ring::Damage1 => self.damage += 1,
                Ring::Damage2 => self.damage += 2,
                Ring::Damage3 => self.damage += 3,
                _ => {}
            }
        }
    }

    fn update_defence_stat(&mut self) {
        self.defence = 0;
        match self.armor {
            Armor::None => {}
            Armor::Leather => self.defence += 1,
            Armor::Chainmail => self.defence += 2,
            Armor::Splintmail => self.defence += 3,
            Armor::Bandedmail => self.defence += 4,
            Armor::Platemail => self.defence += 5,
        }
        for ring in self.rings.iter() {
            match ring {
                Ring::Defence1 => self.defence += 1,
                Ring::Defence2 => self.defence += 2,
                Ring::Defence3 => self.defence += 3,
                _ => {}
            }
        }
    }

    fn change_weapon(&mut self, weapon: Weapon) {
        self.weapon = weapon;
        self.update_damage_stat();
    }

    fn change_armor(&mut self, armor: Armor) {
        self.armor = armor;
        self.update_defence_stat();
    }

    fn change_rings(&mut self, rings: Vec<Ring>) {
        self.rings = rings;
        self.update_damage_stat();
        self.update_defence_stat();
    }

    fn attack(&self, boss: &mut Boss) {
        let damage = self.damage.checked_sub(boss.defence);
        match damage {
            Some(damage) => boss.take_damage(damage),
            None => boss.take_damage(1),
        }
    }

    fn take_damage(&mut self, damage: u32) {
        if damage > self.hp {
            self.hp = 0;
        } else {
            self.hp -= damage;
        }
    }
}

struct Boss {
    damage: u32,
    defence: u32,
    hp: u32,
}

impl Boss {
    fn new(damage: u32, defence: u32, hp: u32) -> Self {
        Self {
            damage,
            defence,
            hp,
        }
    }

    fn take_damage(&mut self, damage: u32) {
        if damage > self.hp {
            self.hp = 0;
        } else {
            self.hp -= damage;
        }
    }

    fn attack(&self, player: &mut Player) {
        let damage = self.damage.checked_sub(player.defence);
        match damage {
            Some(damage) => player.take_damage(damage),
            None => player.take_damage(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke("");
        assert_eq!(result, "0");
    }
}
