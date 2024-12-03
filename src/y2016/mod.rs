mod t01x01;
mod t01x02;
mod t02x01;
mod t02x02;
mod t03x01;
mod t03x02;

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
		_ => {
			println!("Task not recognised")
		}
	}
}
