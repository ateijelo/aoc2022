use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{self, BufRead}, cmp::max,
};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    // name: String,
    rate: u32,
    neighbors: Vec<String>,
}

type Graph = HashMap<String, Valve>;
type DistanceMap = HashMap<(String, String), Option<usize>>;

fn parse_input(lines: &[String]) -> Graph {
    let mut valves = HashMap::new();
    let re = Regex::new(r"Valve (?P<valve>\w+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<neighbors>[A-Z, ]+)")
        .unwrap();
    for line in lines {
        let caps = re.captures(line).unwrap();
        let name = caps.name("valve").unwrap().as_str().to_owned();
        let rate: u32 = caps.name("rate").unwrap().as_str().parse().unwrap();
        let neighbors: Vec<String> = caps
            .name("neighbors")
            .unwrap()
            .as_str()
            .split(", ")
            .map(String::from)
            .collect();
        valves.insert(
            name.clone(),
            Valve {
                rate,
                neighbors,
            },
        );
    }
    // println!("valves: {:?}", valves);
    valves
}

// fn walk(
//     time: usize,
//     graph: &Graph,
//     valve: String,
//     visited: &mut HashSet<String>,
//     opened: &mut HashSet<String>,
// ) {
//     let indent = " ".repeat(time);
//     // println!("visited: {:?}", visited);
//     visited.insert(valve.clone());
//     let v = graph.get(&valve).unwrap();
//     for neighbor in &v.neighbors {
//         // let neighbor = graph.get(&name).expect(&format!("graph should have valve named {}", name));
//         if visited.contains(neighbor) {
//             continue;
//         }
//         println!("{}going from {} to {}", indent, valve, neighbor);
//         walk(time + 1, graph, neighbor.clone(), visited, opened);
//         println!("{}returning from {} to {}", indent, neighbor, valve);
//     }
//     visited.remove(&valve);
//     // let actions = ... visit neighbors & open current valve if rate != 0
//     // for action in actions {
//     //     walk ( state + action );
//     // }
// }

fn compute_pressure_release(path: &Vec<String>, graph: &Graph, distance_map: &DistanceMap) -> u32 {
    let mut release = 0u32;
    let mut rate = 0;
    let mut time = 0;
    for i in 1..path.len() {
        let f = path[i - 1].clone();
        let t = path[i].clone();
        let d = distance_map.get(&(f, t.clone())).unwrap().unwrap() as u32;

        // while I walk from `f` to `t`, release increases by rate + distance
        time += d;
        release += rate * d;

        // when I arrive to `t`, I spend 1 minute opening the valve
        time += 1;
        release += rate;

        // and rate increases by `t`'s rate
        let valve = graph.get(&t).unwrap();
        rate += valve.rate;
    }
    // if we have time to spare, release will increase until we get to 26
    if time < 26 {
        release += rate * (26 - time)
    }
    release
}

fn walk(
    path: &mut Vec<String>,
    time: u32,
    vector: &mut [String],
    graph: &Graph,
    distance_map: &DistanceMap,
    result: &mut u32,
) {
    let r = compute_pressure_release(path, graph, distance_map);
    *result = max(*result, r);
    if path.len() == vector.len() {
        // println!("{:?}", path);
        return;
    }
    for i in 0..vector.len() {
        if path.contains(&vector[i]) {
            continue;
        }

        let from = path.last().unwrap().clone();
        let to = vector[i].clone();

        // we spend `distance` minutes to get to the valve
        let key = (from, to.clone());
        let mut time_spent = distance_map.get(&key).unwrap().unwrap() as u32;
        // and 1 minute in this valve:
        time_spent += 1;

        if time + time_spent > 26 {
            continue;
        }

        path.push(to.clone());
        walk(
            path,
            time + time_spent,
            vector,
            graph,
            distance_map,
            result,
        );
        path.pop();
    }
}

fn successors(valve: &str, graph: &Graph) -> Vec<(String, usize)> {
    if let Some(v) = graph.get(valve) {
        return v
            .neighbors
            .iter()
            .map(|neighbor| (neighbor.to_string(), 1))
            .collect();
    }
    vec![]
}

fn distance_map(
    graph: &Graph,
    skip: &HashSet<&String>,
) -> HashMap<(String, String), Option<usize>> {
    let mut result = HashMap::new();

    for a in graph.keys() {
        for b in graph.keys() {
            if skip.contains(a) {
                continue;
            };
            if skip.contains(b) {
                continue;
            };
            let distance =
                dijkstra(a, |valve| successors(valve, graph), |valve| valve == b).map(|r| r.1);
            result.insert((a.clone(), b.clone()), distance);
        }
    }
    result
}

fn solution(graph: Graph) -> u32 {
    let mut skip = HashSet::new();
    let mut good_valves = vec![];
    for valve in graph.keys() {
        if valve != "AA" && graph.get(valve).unwrap().rate == 0 {
            skip.insert(valve);
        } else {
            good_valves.push(valve.clone());
        }
    }
    let dm = distance_map(&graph, &skip);

    let mut result = 0;
    for i in 1..=(good_valves.len() - 1) / 2 {
        for my_valves in good_valves.iter().filter(|v| *v != "AA").combinations(i) {
            let mut elephant_valves: Vec<String> = good_valves.iter().filter(|v| !my_valves.contains(v)).cloned().collect();
            let mut my_valves: Vec<String> = my_valves.iter().map(|x| (*x).clone()).collect();
            my_valves.push("AA".to_owned());
            println!("me: {:?} elephant: {:?}", my_valves, elephant_valves);

            let mut my_result = 0;
            let mut path = vec!["AA".to_owned()];
            walk(&mut path, 0, &mut my_valves, &graph, &dm, &mut my_result);

            let mut path = vec!["AA".to_owned()];
            let mut elephant_result = 0;
            walk(&mut path, 0, &mut elephant_valves, &graph, &dm, &mut elephant_result);

            let total = my_result + elephant_result;
            result = max(total, result);
        }
    }

    result
}

fn solve(lines: &[String]) -> u32 {
    solution(parse_input(lines))
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn test_file(filename: &str, solution: &str) {
        let reader = BufReader::new(File::open(filename).unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "1651");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "1741");
    }
}
