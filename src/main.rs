
use adventofcode2023::day1;
use std::time::SystemTime;

fn solve_with_time_metric(solver: fn()) {
    let now = SystemTime::now();
    solver();
    println!("Took {} microseconds", now.elapsed().unwrap().as_micros());
}

fn main() {
    solve_with_time_metric(day1::solve_part_1);
    solve_with_time_metric(day1::solve_part_2);
}
