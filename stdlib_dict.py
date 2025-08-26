import time
import json
import os

NUM = 1_000_000

keys = ["k"+str(i) for i in range(NUM)]

start = time.time()

# Step 1: Initialize the dictionary
dic = {keys[i]: i % 7 for i in range(NUM)}

# Step 2: Modify values in the dictionary
for i in range(0, NUM, 2):
    dic[keys[i]] *= 2

# Step 3: Calculate the sum of all values
sum_val = sum(dic[keys[i]] for i in range(NUM))

# Calculate elapsed time
elapsed = time.time() - start

# Write results to JSON file
results = {
    "program": "stdlib_dict.py",
    "time_sec": elapsed,
    "sum": sum_val
}

result_file_path = os.path.join("./results", "stdlib_dict-py.json")
with open(result_file_path, "w") as f:
    json.dump(results, f, indent=4)

# Append elapsed time to CSV
csv_path = os.path.join("./results", "stdlib_dict-py.csv")
with open(csv_path, "a") as f:  # "a" mode appends
    f.write(f"{elapsed:.6f}\n")

print(f"Sum: {sum_val}, elapsed sec: {elapsed:.6f}")
