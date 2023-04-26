use max_timeout_calculator::MaxTimeoutCalculator;
use std::{time::Duration};
use text_io::{read};

fn main() {
    println!("Enter min_backoff_seconds: u64");
    let min_backoff_seconds: u64 = read!();
    println!("Enter cap_total_wait_seconds: u64");
    let cap_total_wait_seconds: u64 = read!();
    println!("Enter random_factor: f64");
    let random_factor: f64 = read!();

    let calculator = MaxTimeoutCalculator::new(
        &Duration::from_secs(min_backoff_seconds),
        random_factor,
        &Duration::from_secs(cap_total_wait_seconds),
    );

    println!(
        "The max_timeout_duration for provided input data should be {:#?} seconds",
        calculator.calculate_max_backoff().unwrap().as_secs()
    )
}
