mod t01x01;
mod t01x02;
mod t02x01;
mod t02x02;
mod t03x01;
mod t03x02;
mod t04x01;
mod t04x02;
mod t05x01;
mod t05x02;
mod t06x01;
mod t06x02;
mod t07x01;
mod t07x02;
mod t08x01;
mod t08x02;
mod t09x01;
mod t09x02;
mod t10x01;
mod t11x01;
mod t12x01;
mod t12x02;
mod t13x01;
mod t13x02;
mod t14x01;
mod t14x02;
mod t15x01;
mod t15x02;
mod t16x01;
mod t16x02;
mod t17x01;
mod t17x02;
mod t18x01;
mod t18x02;
mod t19x01;
mod t19x02;
mod t20x01;
mod t21x01;
mod t21x02;

pub fn invoke_task(
	task: String,
	puzzle_input: String,
) {
	match task.as_str() {
		"01x01" => {
			let out = t01x01::invoke(puzzle_input);
			println!("{out}");
		}
		"01x02" => {
			let out = t01x02::invoke(puzzle_input);
			println!("{out}");
		}
		"02x01" => {
			let out = t02x01::invoke(puzzle_input);
			println!("{out}");
		}
		"02x02" => {
			let out = t02x02::invoke(puzzle_input);
			println!("{out}");
		}
		"03x01" => {
			let out = t03x01::invoke(puzzle_input);
			println!("{out}");
		}
		"03x02" => {
			let out = t03x02::invoke(puzzle_input);
			println!("{out}");
		}
		"04x01" => {
			let out = t04x01::invoke(puzzle_input);
			println!("{out}");
		}
		"04x02" => {
			let out = t04x02::invoke(puzzle_input);
			println!("{out}");
		}
		"05x01" => {
			let out = t05x01::invoke(puzzle_input);
			println!("{out}");
		}
		"05x02" => {
			let out = t05x02::invoke(puzzle_input);
			println!("{out}");
		}
		"06x01" => {
			let out = t06x01::invoke(puzzle_input);
			println!("{out}");
		}
		"06x02" => {
			let out = t06x02::invoke(puzzle_input);
			println!("{out}");
		}
		"07x01" => {
			let out = t07x01::invoke(puzzle_input);
			println!("{out}");
		}
		"07x02" => {
			let out = t07x02::invoke(puzzle_input);
			println!("{out}");
		}
		"08x01" => {
			let out = t08x01::invoke(puzzle_input);
			println!("{out}");
		}
		"08x02" => {
			let out = t08x02::invoke(puzzle_input);
			println!("{out}");
		}
		"09x01" => {
			let out = t09x01::invoke(puzzle_input);
			println!("{out}");
		}
		"09x02" => {
			let out = t09x02::invoke(puzzle_input);
			println!("{out}");
		}
		"10x01" => {
			let out = t10x01::invoke(puzzle_input, 40);
			println!("{out}");
		}
		"10x02" => {
			let out = t10x01::invoke(puzzle_input, 50);
			println!("{out}");
		}
		"11x01" => {
			let out = t11x01::invoke(puzzle_input, 1);
			println!("{out}");
		}
		"11x02" => {
			let out = t11x01::invoke(puzzle_input, 2);
			println!("{out}");
		}
		"12x01" => {
			let out = t12x01::invoke(puzzle_input);
			println!("{out}");
		}
		"12x02" => {
			let out = t12x02::invoke(puzzle_input);
			println!("{out}");
		}
		"13x01" => {
			let out = t13x01::invoke(puzzle_input);
			println!("{out}");
		}
		"13x02" => {
			let out = t13x02::invoke(puzzle_input);
			println!("{out}");
		}
		"14x01" => {
			let out = t14x01::invoke(puzzle_input, 2503);
			println!("{out}");
		}
		"14x02" => {
			let out = t14x02::invoke(puzzle_input, 2503);
			println!("{out}");
		}
		"15x01" => {
			let out = t15x01::invoke(puzzle_input);
			println!("{out}");
		}
		"15x02" => {
			let out = t15x02::invoke(puzzle_input);
			println!("{out}");
		}
		"16x01" => {
			let out = t16x01::invoke(puzzle_input);
			println!("{out}");
		}
		"16x02" => {
			let out = t16x02::invoke(puzzle_input);
			println!("{out}");
		}
		"17x01" => {
			let out = t17x01::invoke(puzzle_input, 150);
			println!("{out}");
		}
		"17x02" => {
			let out = t17x02::invoke(puzzle_input, 150);
			println!("{out}");
		}
		"18x01" => {
			let out = t18x01::invoke(puzzle_input, 100);
			println!("{out}");
		}
		"18x02" => {
			let out = t18x02::invoke(puzzle_input, 100);
			println!("{out}");
		}
		"19x01" => {
			let out = t19x01::invoke(puzzle_input);
			println!("{out}");
		}
		"19x02" => {
			let out = t19x02::invoke(puzzle_input);
			println!("{out}");
		}
		"20x01" => {
			let out = t20x01::invoke(puzzle_input);
			println!("{out}");
		}
		"21x01" => {
			let out = t21x01::invoke(puzzle_input);
			println!("{out}");
		}
		"21x02" => {
			let out = t21x02::invoke(puzzle_input);
			println!("{out}");
		}
		_ => {
			println!("Task not recognised")
		}
	}
}
