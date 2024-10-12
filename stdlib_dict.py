import time
import json
import os

NUM = 1_000_000

keys = [None] * NUM

# Populate the list with strings of even numbers
for i in range(NUM):
    keys[i] = "k"+str(i)

start = time.time()

# Step 1: Initialize the dictionary
dic = {keys[i]: i % 7 for i in range(NUM)}

# Step 2: Modify values in the dictionary
for i in range(0,NUM,2):
    dic[keys[i]] *= 2

# Step 3: Calculate the sum of all values
sum_val = sum(dic[keys[i]] for i in range(NUM))

# Calculate elapsed time
elapsed = time.time() - start

# Write results to individual JSON file in results directory
results = {
    "program": "stdlib_dict.py",
    "time_sec": elapsed,
    "sum": sum_val
}

result_file_path = os.path.join("./results", "stdlib_dict-py.json")
with open(result_file_path, "w") as f:
    json.dump(results, f, indent=4)
