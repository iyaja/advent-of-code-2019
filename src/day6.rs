use std::collections::HashMap;


extern crate petgraph;

use petgraph::algo::astar;
use petgraph::graphmap::GraphMap;
use petgraph::visit::Dfs;
use petgraph::visit::Walker;
use petgraph::Directed;
use petgraph::EdgeType;
use petgraph::Undirected;
// use std::fs::read_to_string;

// use std::env;

// #[aoc_generator(day6)]
// pub fn input_generator(input: &str) -> HashMap<String, Vec<String>> {
//     let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
//     for l in input.lines() {
//         let pair: Vec<String> = l.split(")").map(|s| s.to_string()).collect();
//         let orbiter = pair[0].to_string();
//         let orbitee = pair[1].to_string();

//         match orbits.get_mut(&orbiter) {
//             Some(planet_list) => planet_list.push(orbitee),
//             None => {orbits.insert(orbiter, vec![orbitee]);},
//         };
//     }
//     orbits
// }

// #[aoc(day6, part1)]
// pub fn orbit_counter(orbits: &HashMap<String, Vec<String>>) -> usize {
//     let mut orbit_count = traverse(orbits, orbits.get("COM").unwrap().to_vec(), 1);
//     println!("{}", orbits.capacity());
//     // for (planet, orbitees) in orbits.iter() {
//     //     // println!("{}", planet);
//     //     // println!("{:?}", orbits.get("COM").unwrap());
//     //     // orbit_count += orbitees.len();
//     // }
//     orbit_count
// }


#[aoc_generator(day6)]
pub fn input_generator_2(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part2)]
pub fn jumps_to_santa(input: &String) -> usize {
    // println!("{}", contains(orbits, orbits.get("COM").unwrap().to_vec(), &"ZB8".to_string()));
    // println!("{}", smallest_parent(orbits, orbits.get("COM").unwrap().to_vec(), &"YOU".to_string(), &"YOU".to_string(), &"COM".to_string()));
    // 0;

    // let input = read_to_string("input/2019/day6.txt").unwrap();
    //println!("{:?}", env::current_dir());
	//let input = read_to_string("input/2019/day6.txt").unwrap();
    let edges: Vec<_> = input.lines().collect();
    let path = calculate_path(edges.clone());
    path
}

pub fn traverse(orbits: &HashMap<String, Vec<String>>, orbit_list: Vec<String>, depth: usize) -> usize {
    let mut sum_orbits = depth * orbit_list.len();
    for planet in orbit_list {
        match orbits.get(&planet) {
            Some(orbit_list) => {sum_orbits += traverse(orbits, orbit_list.to_vec(), depth + 1);},
            None => {}
        }
    }    
    sum_orbits
}

pub fn contains(orbits: &HashMap<String, Vec<String>>, orbit_list: Vec<String>, node: &String) -> bool {
    let mut found = false;
    if orbit_list.contains(&node) {
        return true;
    } else {
        for planet in orbit_list {
            match orbits.get(&planet) {
                Some(orbit_list) => { found = found | contains(orbits, orbit_list.to_vec(), node) ;},
                None => {found = found | false}
            }
        }
    }
    found
}

pub fn smallest_parent(orbits: &HashMap<String, Vec<String>>, orbit_list: Vec<String>, a: &String, b: &String, parent: &String) -> String {
    for planet in orbit_list {
        match orbits.get(&planet) {
            Some(orbit_list) => {
                if contains(orbits, orbit_list.to_vec(), a) && contains(orbits, orbit_list.to_vec(), b) {
                    return smallest_parent(orbits, orbit_list.to_vec(), a, b, &planet);
                } else {
                    return String::from(parent);
                }
            },
            None => {}
        }
    }
    String::from("hee")
}

fn build_graph<D: EdgeType>(edges: Vec<&str>) -> GraphMap<&str, (), D> {
	let mut graph = GraphMap::<_, (), D>::new();
	for edge in edges {
		let from_to: Vec<_> = edge.split(')').collect();
		graph.add_edge(from_to[0], from_to[1], ());
	}
	graph
}

fn calculate_indirect(edges: Vec<&str>) -> usize {
	let graph = build_graph::<Directed>(edges);
	graph
		.nodes()
		.map(|node| Dfs::new(&graph, node).iter(&graph).count() - 1)
		.sum()
}

fn calculate_path(edges: Vec<&str>) -> usize {
	let graph = build_graph::<Undirected>(edges);
	let path = astar(&graph, "YOU", |f| f == "SAN", |_| 1, |_| 0).unwrap();
	path.1.len() - 3
}