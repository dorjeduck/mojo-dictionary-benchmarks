use ahash::AHashMap;
use std::time::Instant;
use std::fs::{File};
use std::io::Write;
use std::path::Path;
use std::env;
use std::fs::OpenOptions;

const NUM: usize = 1_000_000;

fn main() {
    let mut keys = vec![String::new(); NUM];

    for i in 0..NUM {
        keys[i] = format!("k{}", i);
    }

    let start = Instant::now();

    let mut dic: AHashMap<String, i32> = keys
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
    let (program_name, json_file_name, csv_file) = if args.contains(&"--optimized".to_string()) {
        ("a_hashmap.rs (optimized)", "a_hashmap_optimized-rs.json", "a_hashmap_optimized-rs.csv")
    } else {
        ("a_hashmap.rs", "a_hashmap-rs.json", "a_hashmap-rs.csv")
    };

    let results_dir_path = Path::new("results").join(json_file_name);
    let results = format!(r#"{{"program": "{}", "time_sec": {:.6}, "sum": {}}}"#, program_name, elapsed, sum_val);
    let mut file = File::create(results_dir_path).unwrap();
    writeln!(file, "{}", results).unwrap();

    // Append elapsed time to CSV
    let csv_path = Path::new("results").join(csv_file);
    let mut file = OpenOptions::new()
        .create(true)    // create if not exists
        .append(true)    // append instead of overwrite
        .open(csv_path)
        .unwrap();

    writeln!(file, "{:.6}", elapsed).unwrap();

    println!("Sum: {}, elapsed sec: {:.6}", sum_val, elapsed);
}