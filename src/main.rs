use std::{fs, path::PathBuf};

use clap::Parser;

mod tasks;

#[derive(Parser, Debug)]
struct Args {
	// Task
	#[arg(short, long)]
	task: String,

	#[arg(short, long)]
	file: PathBuf,

	#[arg(short, long)]
	debug: bool,
}

fn main() {
	println!("My Advent of Code");
	let args = Args::parse();
	println!("{:?}", args);

	let input = fs::read_to_string(args.file).expect("Should have been able to read the file");
	let task = args.task.as_str();

	match task {
		"01x01" => {
			let out = tasks::t01x01::invoke(input, args.debug);
			println!("{out}");
		}
		"01x02" => {
			let out = tasks::t01x02::invoke(input, args.debug);
			println!("{out}");
		}
		"02x01" => {
			let out = tasks::t02x01::invoke(input, args.debug);
			println!("{out}");
		}
		"02x02" => {
			let out = tasks::t02x02::invoke(input, args.debug);
			println!("{out}");
		}
		"03x01" => {
			let out = tasks::t03x01::invoke(input, args.debug);
			println!("{out}");
		}
		"03x03" => {
			let out = tasks::t03x02::invoke(input, args.debug);
			println!("{out}");
		}
		"04x01" => {
			let out = tasks::t04x01::invoke(input, args.debug);
			println!("{out}");
		}
		"04x02" => {
			let out = tasks::t04x02::invoke(input, args.debug);
			println!("{out}");
		}
		"05x01" => {
			let out = tasks::t05x01::invoke(input);
			println!("{out}");
		}
		"05x02" => {
			let out = tasks::t05x02::invoke(input);
			println!("{out}");
		}
		"06x01" => {
			let out = tasks::t06x01::invoke(input);
			println!("{out}");
		}
		"06x02" => {
			let out = tasks::t06x02::invoke(input);
			println!("{out}");
		}
		"07x01" => {
			let out = tasks::t07x01::invoke(input);
			println!("{out}");
		}
		"07x02" => {
			let out = tasks::t07x02::invoke(input);
			println!("{out}");
		}
		"08x01" => {
			let out = tasks::t08x01::invoke(input, args.debug);
			println!("{out}");
		}
		"08x02" => {
			let out = tasks::t08x02::invoke(input, args.debug);
			println!("{out}");
		}
		"09x01" => {
			let out = tasks::t09x01::invoke(input, args.debug);
			println!("{out}");
		}
		"10x01" => {
			let out = tasks::t10x01::invoke(input, args.debug);
			println!("{out}");
		}
		"10x02" => {
			let out = tasks::t10x02::invoke(input, args.debug);
			println!("{out}");
		}
		"11x01" => {
			let out = tasks::t11x01::invoke(input, args.debug);
			println!("{out}");
		}
		"11x02" => {
			let out = tasks::t11x02::invoke(input, 1_000_000, args.debug);
			println!("{out}");
		}
		"12x01" => {
			let out = tasks::t12x01::invoke(input, args.debug);
			println!("{out}");
		}
		_ => {
			println!("Task not recognised")
		}
	}
}
