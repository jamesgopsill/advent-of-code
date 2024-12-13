use std::collections::HashMap;

use regex::Regex;

type Location = String;
type Distance = u32;
type RouteMap = HashMap<Location, HashMap<Location, Distance>>;

pub fn invoke(input: &String) -> String {
	let re = Regex::new(r"(\w+)\sto\s(\w+)\s=\s(\d+)").unwrap();
	let mut route_map: RouteMap = HashMap::new();
	// Create the route map
	for line in input.lines() {
		let captured = re.captures(line);
		match captured {
			Some(cap) => {
				let u = cap.get(1).unwrap().as_str().to_string();
				let v = cap.get(2).unwrap().as_str().to_string();
				let d = cap.get(3).unwrap().as_str();
				let d = d.parse::<u32>().unwrap();
				if let Some(route) = route_map.get_mut(&u) {
					route.insert(v.clone(), d);
				} else {
					let mut map = HashMap::new();
					map.insert(v.clone(), d);
					route_map.insert(u.clone(), map);
				}
				if let Some(route) = route_map.get_mut(&v) {
					route.insert(u.clone(), d);
				} else {
					let mut map = HashMap::new();
					map.insert(u.clone(), d);
					route_map.insert(v.clone(), map);
				}
			}
			_ => {
				panic!("Failed capture")
			}
		}
	}
	//println!("{:?}", routes);
	// TODO: Now to recursively traverse the map.
	let mut routes: Vec<(Vec<String>, u32)> = vec![];
	for location in route_map.keys() {
		routes.push((vec![location.clone()], 0));
	}
	let routes = traverse(routes, route_map, 0);
	let mut min_dist = routes[0].1;
	for (route, dist) in routes {
		if min_dist > dist {
			println!("New Min Dist Route");
			println!("{:?}", route);
			min_dist = dist;
		}
	}
	min_dist.to_string()
}

fn traverse(
	routes: Vec<(Vec<String>, u32)>,
	route_map: RouteMap,
	n: u32,
) -> Vec<(Vec<String>, u32)> {
	/*
	if n > 5 {
		return previous_routes;
	}
	*/
	let mut updated_routes: Vec<(Vec<String>, u32)> = vec![];
	let mut traverse_again: bool = false;
	for (route, route_dist) in routes {
		let current_location = route.last().unwrap();
		let connections = route_map.get(current_location).unwrap();
		for (location, path_dist) in connections {
			if route.contains(location) {
				// Keep it only if we have visited everywhere.
				if route.len() == route_map.len() {
					updated_routes.push((route.clone(), route_dist));
				}
			} else {
				// Add the location to the route
				let mut appended_route = route.clone();
				appended_route.push(location.clone());
				let dist = route_dist + path_dist;
				updated_routes.push((appended_route, dist));
				traverse_again = true;
			}
		}
	}
	if traverse_again {
		let n = n + 1;
		return traverse(updated_routes, route_map, n);
	}
	return updated_routes;
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"
			.to_string(),
		);
		assert_eq!(result, "605");
	}
}
