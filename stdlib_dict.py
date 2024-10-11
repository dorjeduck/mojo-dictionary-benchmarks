import time
import json
import os

NUM = 1_000_000

# Hardcoded results directory
results_dir = "results"
os.makedirs(results_dir, exist_ok=True)

start = time.time()

# Step 1: Initialize the dictionary
dic = {str(i * 2): i % 7 for i in range(NUM)}

# Step 2: Modify values in the dictionary
for i in range(0,NUM,2):
    dic[str(i * 2)] *= 2

# Step 3: Calculate the sum of all values
sum_val = sum(dic[str(i * 2)] for i in range(NUM))

# Calculate elapsed time
elapsed = time.time() - start

# Write results to individual JSON file in results directory
results = {
    "program": "stdlib_dict.py",
    "time_sec": elapsed,
    "sum": sum_val
}

result_file_path = os.path.join(results_dir, "stdlib_dict-py.json")
with open(result_file_path, "w") as f:
    json.dump(results, f, indent=4)