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
		_ => {
			println!("Task not recognised")
		}
	}
}
