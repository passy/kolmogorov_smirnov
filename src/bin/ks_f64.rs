extern crate kolmogorov_smirnov as ks;

use ks::test_f64;

use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

fn parse_float(s: String) -> f64 {
    s.parse::<f64>().expect("Not a floating point number.")
}

/// Runs a Kolmogorov-Smirnov test on floating point data files.
///
/// Input files must be single-column headerless data files. The data samples
/// are tested against each other at the 0.95 confidence level.
///
/// # Examples
///
/// ```bash
/// cargo run --bin ks_f64 <file1> <file2>
/// ```
///
/// This will print the test result to standard output.
fn main() {
    let args: Vec<String> = env::args().collect();

    let path1 = Path::new(&args[1]);
    let path2 = Path::new(&args[2]);

    let file1 = BufReader::new(File::open(&path1).unwrap());
    let file2 = BufReader::new(File::open(&path2).unwrap());

    let lines1 = file1.lines().map(|line| line.unwrap());
    let lines2 = file2.lines().map(|line| line.unwrap());

    let xs: Vec<f64> = lines1.map(parse_float).collect();
    let ys: Vec<f64> = lines2.map(parse_float).collect();

    let result = ks::test_f64(&xs, &ys, 0.95);

    if result.is_rejected {
        println!("Samples are from different distributions.");
    } else {
        println!("Samples are from the same distributions.");
    }

    println!("test statistic = {}", result.statistic);
    println!("critical value = {}", result.critical_value);
    println!("confidence = {}", result.confidence);
}
