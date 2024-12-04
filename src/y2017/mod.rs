mod t01x01;

pub fn invoke_task(
	task: String,
	puzzle_input: String,
) {
	match task.as_str() {
		"01x01" => {
			let out = t01x01::invoke(&puzzle_input);
			println!("{out}");
		}
		_ => {
			println!("Task not recognised")
		}
	}
}
