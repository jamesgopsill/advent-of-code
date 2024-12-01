pub fn invoke(input: String) -> u32 {
	let presents = input.trim().parse::<u32>().unwrap();
	println!("{}", presents);

	let mut low: u32 = 1;
	let mut high: u32 = 10_000_000;

	for _ in 1..100 {
		let mut house_number = (high + low) / 2;
		if house_number % 2 != 0 {
			house_number -= 1;
		}
		let mut house_presents = 0;
		for elf in 1..=house_number {
			if house_number % elf == 0 {
				house_presents += elf * 10;
			}
		}
		if house_presents == presents {
			return house_number;
		}
		if house_presents > presents {
			println!("Too high");
			high = house_number;
		}
		if house_presents < presents {
			println!("Too low");
			low = house_number;
		}
		println!("{} {} {} {}", low, high, house_number, house_presents);
		if high == low + 2 {
			break;
		}
	}

	println!("----");
	let mut low_presents = 0;
	for elf in 1..=low {
		if low % elf == 0 {
			low_presents += elf * 10;
		}
	}
	println!("{} {}", low, low_presents);
	let mut high_presents = 0;
	for elf in 1..=high {
		if high % elf == 0 {
			high_presents += elf * 10;
		}
	}
	println!("{} {}", high, high_presents);
	let mid = high - 1;
	let mut mid_presents = 0;
	for elf in 1..=mid {
		if mid % elf == 0 {
			mid_presents += elf * 10;
		}
	}
	println!("{} {}", mid, mid_presents);

	0
}
