use std::time::Instant;

pub fn bench(
	f: fn(&String) -> u32,
	p: &String,
) {
	let mut trials: Vec<u128> = vec![];
	for _ in 0..100 {
		let now = Instant::now();
		f(p);
		let us = now.elapsed().as_micros();
		trials.push(us)
	}
	println!("Mean: {}", mean(&trials));
	println!("Max: {:?}", trials.iter().max().unwrap());
	println!("Min: {:?}", trials.iter().min().unwrap());
}

fn mean(numbers: &[u128]) -> u128 {
	numbers.iter().sum::<u128>() / numbers.len() as u128
}
