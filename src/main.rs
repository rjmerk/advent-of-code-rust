use std::env;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use itertools::sorted;

mod advent_of_code_2018;

use advent_of_code_2018::init_advent_code_map;


fn main() {
    /*
    How to call:

    to run all problems
    cargo run --release

    To run specific problem
    cargo run <int> --release
 */
    let problems = init_advent_code_map();
    for nr in sorted(problems.keys()) {
        solve_problem(*nr, &problems);
    }

    let args: Vec<String> = env::args()
        .filter(|x| x != "--release")
        .collect();

    if args.len() == 1 {
        for nr in sorted(problems.keys()) {
            solve_problem(*nr, &problems);
        }
    } else {
        let problem_nr = args[1].parse::<i32>();
        match problem_nr {
            Ok(nr) => solve_problem(nr, &problems),
            Err(error) => println!("Could not parse number: {}", error)
        };
    }
}

fn solve_problem(problem_nr: i32, problems: &HashMap<i32, fn()-> ()>) {
    println!();
    println!("Problem {}", problem_nr);
    let now: Instant = Instant::now();
    problems[&problem_nr]();
    let duration: Duration = now.elapsed();
    println!("Time in milliseconds: {:>6.2}", duration.as_secs_f64() * 1000 as f64);
}
