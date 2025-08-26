use std::time::Instant;
use std::fs::{File};
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

#[path="../dict.rs"]
mod dict;
use dict::Dict;

const N: usize = 1_000_000;

fn main() {
    let keys: Vec<String> = (0..N).map(|i| format!("k{}", i)).collect();

    let start = Instant::now();

    let mut dic: Dict<i32, dict::ahash::FxStrHash> = Dict::new(N);
    // Insert
    for (i, key) in keys.iter().enumerate() {
        dic.put(&key, (i % 7) as i32);
    }

    // Update every 2nd key
    for (_, key) in keys.iter().enumerate().step_by(2) {
        let val = dic.get_or(key, 0);
        dic.put(&key, val * 2);
    }

    // let duration_add = start.elapsed().as_secs_f64() * 1000.0;
    // println!("Adding {} keys took: {:.3} ms", N, duration_add);

    let mut sum_val = 0;
    for key in &keys {
        sum_val += dic.get_or(key, -1); // .copied().unwrap_or(-1);
    }
    // let duration_get = start.elapsed().as_secs_f64() * 1000.0;

    // Calculate elapsed time
    let elapsed = start.elapsed().as_secs_f64();

    // Write results to JSON file in results directory
    let (program_name, file_name) = ("compact_dict_fx.rs", "compact-dict-fx-rs.json");

    let results_dir_path = Path::new("results").join(file_name);
    let results = format!(r#"{{"program": "{}", "time_sec": {:.6}, "sum": {}}}"#, program_name, elapsed, sum_val);
    let mut file = File::create(results_dir_path).unwrap();
    writeln!(file, "{}", results).unwrap();

    // Append elapsed time to CSV
    let csv_path = Path::new("results").join("compact-dict-fx-rs.csv");
    let mut file = OpenOptions::new()
        .create(true)    // create if not exists
        .append(true)    // append instead of overwrite
        .open(csv_path)
        .unwrap();

    writeln!(file, "{:.6}", elapsed).unwrap();

    println!("Sum: {}, elapsed sec: {:.6}", sum_val, elapsed);
}