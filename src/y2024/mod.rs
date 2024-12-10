use crate::{bench::bench_u32, bench::bench_u64};

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
mod t10x02;

pub fn invoke_task(
	task: String,
	puzzle_input: String,
) {
	match task.as_str() {
		"01x01" => {
			let out = t01x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t01x01::invoke, &puzzle_input);
		}
		"01x02" => {
			let out = t01x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t01x02::invoke, &puzzle_input);
		}
		"02x01" => {
			let out = t02x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t02x01::invoke, &puzzle_input);
		}
		"02x02" => {
			let out = t02x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t02x02::invoke, &puzzle_input);
		}
		"03x01" => {
			let out = t03x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t03x01::invoke, &puzzle_input);
		}
		"03x02" => {
			let out = t03x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t03x02::invoke, &puzzle_input);
		}
		"04x01" => {
			let out = t04x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t04x01::invoke, &puzzle_input);
		}
		"04x02" => {
			let out = t04x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t04x02::invoke, &puzzle_input);
		}
		"05x01" => {
			let out = t05x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t05x01::invoke, &puzzle_input);
		}
		"05x02" => {
			let out = t05x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t05x02::invoke, &puzzle_input);
		}
		"06x01" => {
			let out = t06x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t06x01::invoke, &puzzle_input);
		}
		"06x02" => {
			let out = t06x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t06x02::invoke, &puzzle_input);
		}
		"07x01" => {
			let out = t07x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u64(t07x01::invoke, &puzzle_input);
		}
		"07x02" => {
			let out = t07x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u64(t07x02::invoke, &puzzle_input);
		}
		"08x01" => {
			let out = t08x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t08x01::invoke, &puzzle_input);
		}
		"08x02" => {
			let out = t08x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t08x02::invoke, &puzzle_input);
		}
		"09x01" => {
			let out = t09x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u64(t09x01::invoke, &puzzle_input);
		}
		"09x02" => {
			let out = t09x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u64(t09x02::invoke, &puzzle_input);
		}
		"10x01" => {
			let out = t10x01::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t10x01::invoke, &puzzle_input);
		}
		"10x02" => {
			let out = t10x02::invoke(&puzzle_input);
			println!("{out}");
			bench_u32(t10x02::invoke, &puzzle_input);
		}
		_ => {
			println!("Task not recognised")
		}
	}
}
