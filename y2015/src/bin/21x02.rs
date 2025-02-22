use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/13.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

pub fn invoke(_: &str) -> String {
	let mut players: Vec<Character> = vec![];
	for weapon in Weapon::iter() {
		for armor in Armor::iter() {
			let p = Character::new(weapon, armor, vec![], 100);
			players.push(p);
			for ring in Ring::iter() {
				let p = Character::new(weapon, armor, vec![ring], 100);
				players.push(p);
			}
			for rings in Ring::iter().combinations(2) {
				let p = Character::new(weapon, armor, rings, 100);
				players.push(p);
			}
		}
	}

	let mut bosses = players.clone();
	for b in bosses.iter_mut() {
		b.max_hp = 104;
		b.current_hp = 104;
	}

	println!("Players: {}, Bosses: {}", players.len(), bosses.len());

	let mut cost: u32 = 0;
	for (i, p) in players.iter_mut().enumerate() {
		println!("{}", i);
		for b in bosses.iter_mut() {
			let c = fight(p, b);
			if c.is_some() {
				let c = c.unwrap();
				println!("I just spent £{}", c);
				if c > cost {
					cost = c;
				}
			}
		}
	}
	println!("Most Expensive Fight: £{}", cost);

	0.to_string()
}

fn fight(
	player: &mut Character,
	boss: &mut Character,
) -> Option<u32> {
	player.current_hp = player.max_hp;
	boss.current_hp = boss.max_hp;
	//println!("Fight Started");
	loop {
		player.attack(boss);
		boss.attack(player);
		//println!("{} {}", player.current_hp, boss.current_hp);
		if boss.current_hp == 0 {
			//println!("Player Wins");
			return None;
		}
		if player.current_hp == 0 {
			//println!("Boss Wins");
			return Some(player.cost() + boss.cost());
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

#[derive(Debug, Clone)]
struct Character {
	weapon: Weapon,
	armor: Armor,
	rings: Vec<Ring>,
	max_hp: u32,
	current_hp: u32,
	damage: u32,
	defence: u32,
}

impl Character {
	fn new(
		weapon: Weapon,
		armor: Armor,
		rings: Vec<Ring>,
		max_hp: u32,
	) -> Self {
		let mut s = Self {
			weapon,
			armor,
			rings,
			max_hp,
			current_hp: max_hp,
			damage: 0,
			defence: 0,
		};
		s.damage_stat();
		s.defence_stat();
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

	fn damage_stat(&mut self) {
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

	fn defence_stat(&mut self) {
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

	fn attack(
		&self,
		c: &mut Character,
	) {
		let damage = self.damage.checked_sub(c.defence);
		match damage {
			Some(damage) => {
				if damage == 0 {
					c.take_damage(1);
				} else {
					c.take_damage(damage);
				}
			}
			None => c.take_damage(1),
		}
	}

	fn take_damage(
		&mut self,
		damage: u32,
	) {
		if damage > self.current_hp {
			self.current_hp = 0;
		} else {
			self.current_hp -= damage;
		}
	}
}

#[cfg(test)]
mod tests_2102 {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("");
		assert_eq!(result, "1");
	}
}
