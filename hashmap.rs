use std::collections::HashMap;
use std::env;
use std::fs::{File};
use std::path::Path;
use std::time::Instant;
use std::io::Write;

const NUM: usize = 1_000_000;

fn main() {
    let mut keys = vec![String::new(); NUM];

    // Populate the list with strings of even numbers
    for i in 0..NUM {
        keys[i] = format!("k{}", i);
    }

    let start = Instant::now();

    // Step 1: Initialize the dictionary
    let mut dic: HashMap<String, i32> = keys
        .iter()
        .enumerate()
        .map(|(i, key)| (key.clone(), (i % 7) as i32))
        .collect();

    // Step 2: Modify values in the dictionary
    for i in (0..NUM).step_by(2) {
        if let Some(val) = dic.get_mut(&keys[i]) {
            *val *= 2;
        }
    }

    // Step 3: Calculate the sum of all values
    let sum_val: i32 = dic.values().sum();

    // Calculate elapsed time
    let elapsed = start.elapsed().as_secs_f64();

    // Write results to JSON file in results directory
    let args: Vec<String> = env::args().collect();
    let (program_name, file_name) = if args.contains(&"--optimized".to_string()) {
        ("hashmap.rs (optimized)", "hashmap_optimized-rs.json")
    } else {
        ("hashmap.rs", "hashmap-rs.json")
    };

    // Write results to JSON file in results directory
    let results_dir = "results";
    let results_dir_path = Path::new(results_dir);
    let result_file_path = results_dir_path.join(file_name);
    let results = format!(r#"{{"program": "{}", "time_sec": {:.6}, "sum": {}}}"#, program_name, elapsed, sum_val);
    let mut file = File::create(result_file_path).unwrap();
    writeln!(file, "{}", results).unwrap();
}