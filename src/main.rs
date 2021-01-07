#![feature(array_windows)]
use std::path::Path;

use anyhow::{bail, ensure, Context};
use clap::{App, Arg};

mod days;
mod solution;

use days::get_solvers;
use solution::Solution;

pub trait Solver {
    fn day(&self) -> u8;
    fn solve(&self, input: &str) -> Solution;
    fn get_input(&self, directory: Option<&str>) -> anyhow::Result<String> {
        let path = input_path(self.day(), directory);
        let input_string = match std::fs::read_to_string(path) {
            Ok(inp) => add_newline(inp.replace("\r", "")),
            Err(error) => bail!("Error while reading input file: {}", error),
        };

        Ok(input_string)
    }
}

fn input_path(day: u8, directory: Option<&str>) -> std::path::PathBuf {
    if let Some(dir) = directory {
        Path::new(dir).join(format!("day{:02}.txt", day))
    } else {
        Path::new("./input").join(format!("day{:02}.txt", day))
    }
}

fn add_newline(mut inp: String) -> String {
    if inp.as_bytes().last() != Some(&b'\n') {
        inp.push('\n');
    };

    inp
}

fn solve(solver: &dyn Solver, input_directory: Option<&str>) -> anyhow::Result<()> {
    let inp = &solver.get_input(input_directory)?;
    let solution = solver.solve(&inp);

    println!("Day {:02}\n{}", solver.day(), solution);

    Ok(())
}

fn solve_all(solvers: Vec<Box<dyn Solver>>, input_directory: Option<&str>) -> anyhow::Result<()> {
    let mut inputs = Vec::new();

    for solver in &solvers {
        let input = solver.get_input(input_directory)?;

        let solution = solver.solve(&input);

        println!("Day {:02}\n{}", solver.day(), solution);

        inputs.push(input);
    }

    Ok(())
}

fn run() -> anyhow::Result<()> {
    let args = App::new("aoc2020")
        .arg(
            Arg::with_name("all")
                .long("all")
                .short("a")
                .help("Solve all days"),
        )
        .arg(
            Arg::with_name("day")
                .long("day")
                .short("d")
                .takes_value(true)
                .help("Solve a single day."),
        )
        .arg(
            Arg::with_name("input-directory")
                .long("input-directory")
                .alias("input-dir")
                .takes_value(true)
                .value_name("DIR")
                .help("Directory for inputs."),
        )
        .get_matches();

    let input_dir = args.value_of("input-directory");

    let solvers = get_solvers();

    if args.is_present("all") {
        solve_all(solvers, input_dir)
    } else if let Some(day) = args.value_of("day") {
        let day = day.parse::<usize>().context("Day is not a valid number")?;
        ensure!(
            1 <= day && day <= solvers.len(),
            "Day out of range. It must be between 1 and {}.",
            solvers.len()
        );
        solve(&*solvers[day - 1], input_dir)
    } else {
        let day = solvers.len() - 1;
        solve(&*solvers[day], input_dir)
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
    }
}
