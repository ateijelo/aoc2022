use std::{
    fmt::Debug,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
struct OreRobot {
    ore: u32,
}

#[derive(Debug)]
struct ClayRobot {
    ore: u32,
}

#[derive(Debug)]
struct ObsidianRobot {
    ore: u32,
    clay: u32,
}

#[derive(Debug)]
struct GeodeRobot {
    ore: u32,
    obsidian: u32,
}

#[derive(Debug)]
struct Blueprint {
    or: OreRobot,
    cr: ClayRobot,
    br: ObsidianRobot,
    gr: GeodeRobot,
}

impl Blueprint {
    fn from(line: &str) -> Self {
        let re = if line.starts_with("Blueprint") {
            Regex::new(concat!(
                r"Blueprint (?P<number>\d+): ",
                r"Each ore robot costs (?P<oro>\d+) ore. ",
                r"Each clay robot costs (?P<cro>\d+) ore. ",
                r"Each obsidian robot costs (?P<bro>\d+) ore and (?P<brc>\d+) clay. ",
                r"Each geode robot costs (?P<gro>\d+) ore and (?P<grb>\d+) obsidian."
            ))
            .unwrap()
        } else {
            Regex::new(concat!(
                r"or=(?P<oro>\d+)o ",
                r"cr=(?P<cro>\d+)o ",
                r"br=(?P<bro>\d+)o,(?P<brc>\d+)c ",
                r"gr=(?P<gro>\d+)o,(?P<grb>\d+)b"
            ))
            .unwrap()
        };

        let caps = re.captures(line).unwrap();
        let read = |name: &str| -> u32 { caps.name(name).unwrap().as_str().parse().unwrap() };
        Self {
            or: OreRobot { ore: read("oro") },
            cr: ClayRobot { ore: read("cro") },
            br: ObsidianRobot {
                ore: read("bro"),
                clay: read("brc"),
            },
            gr: GeodeRobot {
                ore: read("gro"),
                obsidian: read("grb"),
            },
        }
    }

    fn time_to_build_geode_robot(&self, state: &State) -> Option<u32> {
        if state.ore_robots == 0 {
            return None;
        }
        if state.obsidian_robots == 0 {
            return None;
        }
        let needed_ore = self.gr.ore.saturating_sub(state.ore);
        let needed_obsidian = self.gr.obsidian.saturating_sub(state.obsidian);
        let ore_wait = num::Integer::div_ceil(&needed_ore, &state.ore_robots);
        let obsidian_wait = num::Integer::div_ceil(&needed_obsidian, &state.obsidian_robots);
        Some(std::cmp::max(ore_wait, obsidian_wait))
    }

    fn time_to_build_obsidian_robot(&self, state: &State) -> Option<u32> {
        if state.ore_robots == 0 {
            return None;
        }
        if state.clay_robots == 0 {
            return None;
        }
        let needed_ore = self.br.ore.saturating_sub(state.ore);
        let needed_clay = self.br.clay.saturating_sub(state.clay);
        let ore_wait = num::Integer::div_ceil(&needed_ore, &state.ore_robots);
        let clay_wait = num::Integer::div_ceil(&needed_clay, &state.clay_robots);
        Some(std::cmp::max(ore_wait, clay_wait))
    }

    fn time_to_build_clay_robot(&self, state: &State) -> Option<u32> {
        if state.ore_robots == 0 {
            return None;
        }
        let needed_ore = self.cr.ore.saturating_sub(state.ore);
        Some(num::Integer::div_ceil(&needed_ore, &state.ore_robots))
    }

    fn time_to_build_ore_robot(&self, state: &State) -> Option<u32> {
        if state.ore_robots == 0 {
            return None;
        }
        let needed_ore = self.or.ore.saturating_sub(state.ore);
        Some(num::Integer::div_ceil(&needed_ore, &state.ore_robots))
    }

    fn jump_to_geode_robot(&self, s: &State) -> Option<State> {
        self.time_to_build_geode_robot(s).map(|time| {
            let time = time + 1;
            State {
                ore: s.ore + time * s.ore_robots - self.gr.ore,
                clay: s.clay + time * s.clay_robots,
                obsidian: s.obsidian + time * s.obsidian_robots - self.gr.obsidian,
                geodes: s.geodes + time * s.geode_robots,
                ore_robots: s.ore_robots,
                clay_robots: s.clay_robots,
                obsidian_robots: s.obsidian_robots,
                geode_robots: s.geode_robots + 1,
                time: s.time + time,
            }
        })
    }

    fn jump_to_obsidian_robot(&self, s: &State) -> Option<State> {
        self.time_to_build_obsidian_robot(s).map(|time| {
            let time = time + 1;
            State {
                ore: s.ore + time * s.ore_robots - self.br.ore,
                clay: s.clay + time * s.clay_robots - self.br.clay,
                obsidian: s.obsidian + time * s.obsidian_robots,
                geodes: s.geodes + time * s.geode_robots,

                ore_robots: s.ore_robots,
                clay_robots: s.clay_robots,
                obsidian_robots: s.obsidian_robots + 1,
                geode_robots: s.geode_robots,

                time: s.time + time,
            }
        })
    }

    fn jump_to_clay_robot(&self, s: &State) -> Option<State> {
        self.time_to_build_clay_robot(s).map(|time| {
            let time = time + 1;
            State {
                ore: s.ore + time * s.ore_robots - self.cr.ore,
                clay: s.clay + time * s.clay_robots,
                obsidian: s.obsidian + time * s.obsidian_robots,
                geodes: s.geodes + time * s.geode_robots,

                ore_robots: s.ore_robots,
                clay_robots: s.clay_robots + 1,
                obsidian_robots: s.obsidian_robots,
                geode_robots: s.geode_robots,

                time: s.time + time,
            }
        })
    }

    fn jump_to_ore_robot(&self, s: &State) -> Option<State> {
        self.time_to_build_ore_robot(s).map(|time| {
            let time = time + 1;
            State {
                ore: s.ore + time * s.ore_robots - self.or.ore,
                clay: s.clay + time * s.clay_robots,
                obsidian: s.obsidian + time * s.obsidian_robots,
                geodes: s.geodes + time * s.geode_robots,

                ore_robots: s.ore_robots + 1,
                clay_robots: s.clay_robots,
                obsidian_robots: s.obsidian_robots,
                geode_robots: s.geode_robots,

                time: s.time + time,
            }
        })
    }

    fn max_ore(&self) -> u32 {
        vec![self.or.ore, self.cr.ore, self.br.ore, self.gr.ore]
            .into_iter()
            .max()
            .unwrap()
    }
    fn max_clay(&self) -> u32 {
        self.br.clay
    }
    fn max_obsidian(&self) -> u32 {
        self.gr.obsidian
    }
}

struct Solver<'a> {
    time_limit: u32,
    blueprint: &'a Blueprint,
    // path: Vec<State>,
    max_geodes_found: u32,
    max_ore_needed: u32,
    max_clay_needed: u32,
    max_obsidian_needed: u32,
}

fn sum_between(a: u32, b: u32) -> u32 {
    if b < a {
        return 0;
    }
    let sum_to_b = b * (b + 1) / 2;
    if a == 0 {
        return sum_to_b;
    }
    let sum_to_a = (a - 1) * a / 2;
    sum_to_b - sum_to_a
}

impl<'a> Solver<'a> {
    fn new(blueprint: &'a Blueprint, time_limit: u32) -> Self {
        // let path = Vec::new();
        Self {
            time_limit,
            blueprint,
            // path,
            max_geodes_found: 0,
            max_ore_needed: blueprint.max_ore(),
            max_clay_needed: blueprint.max_clay(),
            max_obsidian_needed: blueprint.max_obsidian(),
        }
    }

    fn solve(&mut self) -> u32 {
        self._solve(State::from("o=0 c=0 b=0 g=0 or=1 cr=0 br=0 gr=0 t=0"));
        self.max_geodes_found
    }

    fn _solve(&mut self, s: State) {
        if s.time > self.time_limit {
            return;
        }

        // assuming I build one geode robot each remaining minute
        let rem = self.time_limit - s.time;
        if rem > 0 {
            let potential_geodes = s.geodes + sum_between(s.geode_robots, s.geode_robots + rem - 1);
            if potential_geodes <= self.max_geodes_found {
                // this branch won't beat the record
                return;
            }
        }

        self.max_geodes_found = std::cmp::max(s.geodes, self.max_geodes_found);

        if let Some(state) = self.blueprint.jump_to_geode_robot(&s) {
            self._solve(state);
            // self.path.pop();
        }
        if s.obsidian_robots < self.max_obsidian_needed {
            if let Some(state) = self.blueprint.jump_to_obsidian_robot(&s) {
                // println!("{:?}", s);
                self._solve(state);
            }
            // self.path.pop();
        }
        if s.clay_robots < self.max_clay_needed {
            if let Some(state) = self.blueprint.jump_to_clay_robot(&s) {
                self._solve(state);
            }
        }
        if s.ore_robots < self.max_ore_needed {
            if let Some(state) = self.blueprint.jump_to_ore_robot(&s) {
                self._solve(state);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    time: u32,
}

impl State {
    fn from(line: &str) -> Self {
        // o=0 c=0 b=0 g=0 or=1 cr=0 br=0 gr=0 t=0
        let re = Regex::new(concat!(
            r"o=(?P<o>\d+) ",
            r"c=(?P<c>\d+) ",
            r"b=(?P<b>\d+) ",
            r"g=(?P<g>\d+) ",
            r"or=(?P<or>\d+) ",
            r"cr=(?P<cr>\d+) ",
            r"br=(?P<br>\d+) ",
            r"gr=(?P<gr>\d+) ",
            r"t=(?P<t>\d+)",
        ))
        .unwrap();

        let caps = re.captures(line).unwrap();
        let read = |name: &str| -> u32 { caps.name(name).unwrap().as_str().parse().unwrap() };
        Self {
            ore: read("o"),
            clay: read("c"),
            obsidian: read("b"),
            geodes: read("g"),
            ore_robots: read("or"),
            clay_robots: read("cr"),
            obsidian_robots: read("br"),
            geode_robots: read("gr"),
            time: read("t"),
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<Blueprint> {
    let mut r = Vec::new();

    for line in lines {
        r.push(Blueprint::from(line));
    }
    r
}

fn solution(blueprints: &[Blueprint]) -> u32 {
    let mut score = 1;
    for (i, blueprint) in blueprints.iter().take(3).enumerate() {
        let mut solver = Solver::new(blueprint, 32);
        let g = solver.solve();
        // println!("{:?}", blueprint);
        score *= g;
        println!("Blueprint {}: {} geodes, score = {}", i + 1, g, score);
    }
    score
}

fn solve(lines: &[String]) -> u32 {
    solution(&parse_input(lines))
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
        test_file("example.txt", "3472");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "15939");
    }

    #[test]
    fn test_ore_robot_jump() {
        let bp = Blueprint::from("or=4o cr=2o br=3o,14c gr=2o,7b");

        let state = State::from("o=0 c=0 b=0 g=0 or=1 cr=0 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_ore_robot(&state), Some(4));

        let state = State::from("o=1 c=0 b=0 g=0 or=1 cr=0 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_ore_robot(&state), Some(3));

        let state = State::from("o=4 c=0 b=0 g=0 or=2 cr=0 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_ore_robot(&state), Some(0));
        assert_eq!(
            bp.jump_to_ore_robot(&state).unwrap(),
            State::from("o=2 c=0 b=0 g=0 or=3 cr=0 br=0 gr=0 t=1")
        );

        let state = State::from("o=2 c=0 b=0 g=0 or=2 cr=0 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_ore_robot(&state), Some(1));
        assert_eq!(
            bp.jump_to_ore_robot(&state).unwrap(),
            State::from("o=2 c=0 b=0 g=0 or=3 cr=0 br=0 gr=0 t=2")
        );
    }

    #[test]
    fn test_obsidian_robot_jump() {
        let bp = Blueprint::from("or=4o cr=2o br=3o,14c gr=2o,7b");

        let state = State::from("o=1 c=0 b=0 g=0 or=1 cr=0 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_obsidian_robot(&state), None);

        let state = State::from("o=3 c=14 b=0 g=0 or=1 cr=1 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_obsidian_robot(&state), Some(0));
        assert_eq!(
            bp.jump_to_obsidian_robot(&state).unwrap(),
            State::from("o=1 c=1 b=0 g=0 or=1 cr=1 br=1 gr=0 t=1")
        );

        let state = State::from("o=1 c=1 b=0 g=0 or=2 cr=2 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_obsidian_robot(&state), Some(7));
        assert_eq!(
            bp.jump_to_obsidian_robot(&state).unwrap(),
            State::from("o=14 c=3 b=0 g=0 or=2 cr=2 br=1 gr=0 t=8")
        );
    }

    #[test]
    fn test_geode_robot_jump() {
        let bp = Blueprint::from("or=4o cr=2o br=3o,14c gr=2o,7b");

        let state = State::from("o=1 c=0 b=0 g=0 or=1 cr=0 br=0 gr=0 t=0");
        assert_eq!(bp.time_to_build_geode_robot(&state), None);

        let state = State::from("o=3 c=0 b=7 g=0 or=1 cr=0 br=1 gr=0 t=0");
        assert_eq!(bp.time_to_build_geode_robot(&state), Some(0));
        assert_eq!(
            bp.jump_to_geode_robot(&state).unwrap(),
            State::from("o=2 c=0 b=1 g=0 or=1 cr=0 br=1 gr=1 t=1")
        );

        let state = State::from("o=0 c=0 b=0 g=0 or=1 cr=0 br=1 gr=0 t=0");
        assert_eq!(bp.time_to_build_geode_robot(&state), Some(7));
        assert_eq!(
            bp.jump_to_geode_robot(&state).unwrap(),
            State::from("o=6 c=0 b=1 g=0 or=1 cr=0 br=1 gr=1 t=8")
        );
    }
}
