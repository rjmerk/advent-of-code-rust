extern crate core;

use itertools::sorted;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

mod advent_of_code_2018;
mod advent_of_code_2022;

use advent_of_code_2018::init_advent_code_map;
use advent_of_code_2022::init_advent_code_map_2022;

fn main() {
    /*
       How to call:

       to run all problems
       cargo run --release

       To run specific problem
       cargo run <int> --release
    */
    init_advent_code_map(); // todo

    let args: Vec<String> = env::args().filter(|x| x != "--release").collect();

    let problems = init_advent_code_map_2022();
    let year = 2022;
    if args.len() == 1 {
        for nr in sorted(problems.keys()) {
            solve_problem(year, *nr, &problems);
        }
    } else {
        let problem_nr = args[1].parse::<i32>();
        match problem_nr {
            Ok(nr) => solve_problem(year, nr, &problems),
            Err(error) => println!("Could not parse number: {}", error),
        };
    }
}

fn solve_problem<E: Error>(
    year: i32,
    problem_nr: i32,
    problems: &HashMap<i32, fn() -> Result<(), E>>,
) {
    println!();
    println!("Advent {}, problem {}", year, problem_nr);
    let problem_to_solve = problems[&problem_nr];
    let now: Instant = Instant::now();
    match problem_to_solve() {
        Ok(_) => {}
        Err(e) => {
            println!("!!! Error happened: {}", e)
        }
    };
    let duration: Duration = now.elapsed();
    println!(
        "Time in milliseconds: {:>6.2}",
        duration.as_secs_f64() * 1000 as f64
    );
}
