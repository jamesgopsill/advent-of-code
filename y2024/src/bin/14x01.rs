#![allow(dead_code)]
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/14.txt").unwrap();
    let out = invoke(&input, 101, 103);
    println!("{out}");
}

fn invoke(input: &str, x_max: i32, y_max: i32) -> String {
    let re = Regex::new(r"([-\d]+),([-\d]+)\sv=([-\d]+),([-\d]+)").unwrap();
    let caps = re.captures_iter(input);
    // Initialise the robots
    let mut robots: Vec<Robot> = vec![];
    for c in caps {
        let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = c.get(4).unwrap().as_str().parse::<i32>().unwrap();
        //println!("{},{} {},{}", x, y, vx, vy);
        let robot = Robot::new(x, y, vx, vy, x_max, y_max);
        robots.push(robot);
    }

    // print a map
    //print_map(&robots, x_max, y_max);
    //println!("-----------");

    for _ in 0..100 {
        for r in robots.iter_mut() {
            r.walk();
        }
    }

    // print a map
    // print_map(&robots, x_max, y_max);

    // Quadrant check.
    let mid_x = x_max / 2;
    let mid_y = y_max / 2;
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    for r in robots.iter() {
        if r.x < mid_x && r.y < mid_y {
            top_left += 1;
            continue;
        }
        if r.x > mid_x && r.y < mid_y {
            top_right += 1;
            continue;
        }
        if r.x > mid_x && r.y > mid_y {
            bottom_right += 1;
            continue;
        }
        if r.x < mid_x && r.y > mid_y {
            bottom_left += 1;
            continue;
        }
    }

    println!("----");
    println!(
        "{} {}\n{} {}",
        top_left, top_right, bottom_left, bottom_right
    );
    let safety_factor = top_left * top_right * bottom_left * bottom_right;
    safety_factor.to_string()
}

fn print_map(robots: &[Robot], x_max: i32, y_max: i32) {
    let mut map = vec![vec![0; x_max as usize]; y_max as usize];
    for r in robots.iter() {
        map[r.y as usize][r.x as usize] += 1;
    }
    for row in map {
        println!("{:?}", row);
    }
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
    fn new(x: i32, y: i32, vx: i32, vy: i32, x_max: i32, y_max: i32) -> Self {
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

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let result = invoke(input, 11, 7);
        assert_eq!(result, "12");
    }
}
