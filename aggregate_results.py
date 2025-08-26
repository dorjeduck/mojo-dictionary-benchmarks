import csv
import json
import math
from pathlib import Path

RESULTS_DIR = Path("results")

def compute_stats(values):
    n = len(values)
    mean = sum(values) / n
    var = sum((x - mean) ** 2 for x in values) / n
    std_dev = math.sqrt(var)
    return mean, std_dev

for csv_file in RESULTS_DIR.glob("*.csv"):
    # Read times from CSV
    times = []
    with open(csv_file, newline='') as f:
        reader = csv.reader(f)
        for row in reader:
            if row: # skip empty lines
                times.append(float(row[0]))

    if not times:
        continue

    mean, std_dev = compute_stats(times)

    # Corresponding JSON file
    json_file = RESULTS_DIR / csv_file.with_suffix(".json").name
    if json_file.exists():
        with open(json_file, "r") as f:
            data = json.load(f)
    else:
        raise f"JSON file {json_file} not found."

    # Update JSON with stats
    data["time_sec_mean"] = mean
    data["time_sec_std"] = std_dev
    data["num_runs"] = len(times)

    with open(json_file, "w") as f:
        json.dump(data, f, indent=4)

    print(f"Updated {json_file.name}: mean={mean:.6f}s, std={std_dev:.6f}s")
