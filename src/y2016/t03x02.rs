pub fn invoke(input: &String) -> String {
	let mut col_1: Vec<u32> = vec![];
	let mut col_2: Vec<u32> = vec![];
	let mut col_3: Vec<u32> = vec![];
	for line in input.lines() {
		let mut items = line.split_whitespace();
		let a = items.next().unwrap().parse::<u32>().unwrap();
		let b = items.next().unwrap().parse::<u32>().unwrap();
		let c = items.next().unwrap().parse::<u32>().unwrap();
		col_1.push(a);
		col_2.push(b);
		col_3.push(c);
	}
	col_1.extend(col_2);
	col_1.extend(col_3);

	let mut valid: u32 = 0;
	for win in col_1.chunks(3) {
		if win[0] + win[1] > win[2] && win[1] + win[2] > win[0] && win[2] + win[0] > win[1] {
			// println!("{}", line);
			valid += 1;
		}
	}
	valid.to_string()
}
