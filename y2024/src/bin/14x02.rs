use regex::Regex;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufWriter, Write},
	path::PathBuf,
	u32,
};

use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/14.txt").unwrap();
	let out = invoke(&input, 101, 103);
	println!("{}", out);
}

fn invoke(
	input: &str,
	x_max: i32,
	y_max: i32,
) -> String {
	let re = Regex::new(r"([-\d]+),([-\d]+)\sv=([-\d]+),([-\d]+)").unwrap();
	let caps = re.captures_iter(input);
	// Initialise the robots
	let mut robots: Vec<Robot> = vec![];
	for c in caps {
		let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
		let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
		let vx = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
		let vy = c.get(4).unwrap().as_str().parse::<i32>().unwrap();
		let robot = Robot::new(x, y, vx, vy, x_max, y_max);
		robots.push(robot);
	}

	for i in 1..10_000 {
		if i % 1_000 == 0 {
			println!("{}", i);
		}
		for r in robots.iter_mut() {
			r.walk();
		}

		let mut n: u32 = 1;
		let mut map = vec![vec![0 as u32; x_max as usize]; y_max as usize];
		for r in robots.iter() {
			map[r.y as usize][r.x as usize] = n;
			n += 1;
		}

		// Clustering
		for _ in 0..10_000 {
			let mut changes: u32 = 0;
			for row in 0..y_max as usize {
				for col in 0..x_max as usize {
					// Disregard 0 (no robot) locations.
					if map[row][col] == 0 {
						continue;
					}
					// top
					if row > 0 && map[row - 1][col] > map[row][col] {
						map[row][col] = map[row - 1][col];
						changes += 1;
					}
					// bottom
					if row < (y_max - 1) as usize && map[row + 1][col] > map[row][col] {
						map[row][col] = map[row + 1][col];
						changes += 1;
					}
					// left
					if col > 0 && map[row][col - 1] > map[row][col] {
						map[row][col] = map[row][col - 1];
						changes += 1;
					}
					// right
					if col < (x_max - 1) as usize && map[row][col + 1] > map[row][col] {
						map[row][col] = map[row][col + 1];
						changes += 1;
					}
				}
			}
			if changes == 0 {
				// println!("Plots Clustered");
				break;
			}
		}

		// Check size of clusters
		let mut sums: HashMap<u32, u32> = HashMap::new();
		for row in 0..y_max as usize {
			for col in 0..x_max as usize {
				let m = map[row][col];
				if m == 0 {
					continue;
				}
				let v = sums.get_mut(&m);
				if v.is_some() {
					let v = v.unwrap();
					*v += 1;
				} else {
					sums.insert(m, 1);
				}
			}
		}

		for (_, v) in sums {
			// Candidate for tree
			if v > 100 {
				let mut path = PathBuf::new();
				path.push("images");
				path.push(format!("walk_{}.svg", i));
				draw_map(&robots, path);
			}
		}
	}

	// walk and check the map
	"".to_string()
}

fn draw_map(
	robots: &Vec<Robot>,
	fpath: PathBuf,
) {
	let file = File::create(fpath);
	let file = file.unwrap();
	let mut writer = BufWriter::new(file);
	writer.write_all(SVG_HEADER.as_bytes()).unwrap();

	for r in robots.iter() {
		let line = format!(
			"<rect x=\"{}\" y=\"{}\" width=\"1\" height=\"1\" fill=\"white\" />\n",
			r.x, r.y,
		);
		writer.write_all(line.as_bytes()).unwrap();
	}

	writer.write_all(SVG_FOOTER.as_bytes()).unwrap();

	writer.flush().unwrap();
}

struct Robot {
	x: i32,
	y: i32,
	vx: i32,
	vy: i32,
	x_max: i32,
	y_max: i32,
}

impl Robot {
	fn new(
		x: i32,
		y: i32,
		vx: i32,
		vy: i32,
		x_max: i32,
		y_max: i32,
	) -> Self {
		Self {
			x,
			y,
			vx,
			vy,
			x_max,
			y_max,
		}
	}

	fn walk(&mut self) {
		self.x += self.vx;
		self.y += self.vy;
		// n.b. not expecting the robot to be faster than
		// the size of the grid.
		if self.x < 0 {
			self.x += self.x_max;
		}
		if self.y < 0 {
			self.y += self.y_max;
		}
		if self.x >= self.x_max {
			self.x -= self.x_max;
		}
		if self.y >= self.y_max {
			self.y -= self.y_max;
		}
	}
}

const SVG_HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<svg width=\"110\" height=\"110\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
<rect x=\"0\" y=\"0\" width=\"110\" height=\"110\" fill=\"black\" />
";

const SVG_FOOTER: &str = "</svg>";
