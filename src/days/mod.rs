#[macro_use]
macro_rules! assert_solver_day {
    ($solver:expr) => {
        assert_eq!(
            format!("src/days/day{:02}.rs", crate::Solver::day(&$solver)),
            file!(),
            "Solver in '{}' has incorrect Solver::day()",
            file!()
        );
    };
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub fn get_solvers() -> Vec<Box<dyn crate::Solver>> {
    vec![
        Box::new(day01::Solver::new()),
        Box::new(day02::Solver::new()),
        Box::new(day03::Solver::new()),
        Box::new(day04::Solver::new()),
        Box::new(day05::Solver::new()),
        Box::new(day06::Solver::new()),
        Box::new(day07::Solver::new()),
        Box::new(day08::Solver::new()),
        Box::new(day09::Solver::new()),
        Box::new(day10::Solver::new()),
        Box::new(day11::Solver::new()),
        Box::new(day12::Solver::new()),
        Box::new(day13::Solver::new()),
        //        Box::new(day14::Solver::new()),
        //        Box::new(day15::Solver::new()),
        //        Box::new(day16::Solver::new()),
        //        Box::new(day17::Solver::new()),
        //        Box::new(day18::Solver::new()),
        //        Box::new(day19::Solver::new()),
        //        Box::new(day20::Solver::new()),
        //        Box::new(day21::Solver::new()),
        //        Box::new(day22::Solver::new()),
        //        Box::new(day23::Solver::new()),
        //        Box::new(day24::Solver::new()),
        //        Box::new(day25::Solver::new()),
    ]
}
