use std::{fs, path::PathBuf, process};

use clap::Parser;

mod y2023;

#[derive(Parser, Debug)]
struct Args {
	#[arg(short, long)]
	year: u32,

	#[arg(short, long)]
	day: u32,

	#[arg(short, long)]
	part: u32,

	#[arg(short, long)]
	file: PathBuf,

	#[arg(long)]
	debug: bool,
}

fn main() {
	println!("Advent of Code CLI");
	let args = Args::parse();
	println!("{:?}", args);

	if args.year > 2023 || args.year < 2015 {
		println!("Year is out of range");
		process::exit(1);
	}

	if args.day > 24 {
		println!("Day is out of range");
		process::exit(1);
	}

	let puzzle_input =
		fs::read_to_string(args.file).expect("Should have been able to read the file");

	let task = format!("{:0>2}x{:0>2}", args.day, args.part);

	match args.year {
		2023 => {
			y2023::invoke_task(task, puzzle_input, args.debug);
		}
		_ => {
			println!("Task not recognised");
			process::exit(1);
		}
	}
}
