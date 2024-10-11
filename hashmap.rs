use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::time::Instant;
use std::path::Path;
use std::env;

const NUM: usize = 1_000_000;

fn main() {
    // Hardcoded results directory
    let results_dir = "results";
    let results_dir_path = Path::new(results_dir);
    if !results_dir_path.exists() {
        create_dir_all(results_dir_path).unwrap();
    }

    let start = Instant::now();

    // Step 1: Initialize the HashMap
    let mut dic: HashMap<usize, usize> = HashMap::with_capacity(NUM);
    dic.extend((0..NUM).map(|i| (i * 2, i % 7)));

    // Step 2: Modify values in the HashMap
    for i in (0..NUM).step_by(2) {
        if let Some(value) = dic.get_mut(&(i * 2)) {
            *value *= 2;
        }
    }

    // Step 3: Calculate the sum of all values
    let sum: usize = (0..NUM).map(|i| dic[&(i * 2)]).sum();

    // Calculate elapsed time
    let elapsed = start.elapsed().as_secs_f64();

    // Check for --optimized flag to change the program name and output file name
    let args: Vec<String> = env::args().collect();
    let (program_name, file_name) = if args.contains(&"--optimized".to_string()) {
        ("hashmap.rs (optimized)", "hashmap_optimized-rs.json")
    } else {
        ("hashmap.rs", "hashmap-rs.json")
    };

    // Write results to JSON file in results directory
    let result_file_path = results_dir_path.join(file_name);
    let results = format!(r#"{{"program": "{}", "time_sec": {:.6}, "sum": {}}}"#, program_name, elapsed, sum);
    let mut file = File::create(result_file_path).unwrap();
    writeln!(file, "{}", results).unwrap();
}
