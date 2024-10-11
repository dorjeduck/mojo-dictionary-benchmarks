import argparse
import json
import matplotlib.pyplot as plt
import os

# Parse command line arguments
parser = argparse.ArgumentParser()
parser.add_argument("--no-show", action="store_true", help="Disable showing the plot")
args = parser.parse_args()

# Result directory
results_dir = "results"

# Read all JSON files in the results directory
results = []
incorrect_files = []
for filename in os.listdir(results_dir):
    if filename.endswith(".json"):
        filepath = os.path.join(results_dir, filename)
        with open(filepath, "r") as f:
            data = json.load(f)
            if data["sum"] != 4499997:
                incorrect_files.append(filename)
            results.append(data)

# Check if all sums are correct
if incorrect_files:
    print(f"The following files have incorrect sums: {incorrect_files}")
    exit(1)
else:
    print("All tests passed.")

# Sort results from fastest to slowest 
results.sort(key=lambda x: x["time_sec"], reverse=True)

# Extract data for plotting
programs = [entry["program"] for entry in results]
times = [entry["time_sec"] for entry in results]

# Write results to an MD file with table view
with open('./results/benchmarks.md', 'w') as md_file:
    md_file.write('# Benchmarks\n\n')
    md_file.write('| Program | Time (seconds) |\n')
    md_file.write('|---------|----------------|\n')
    for entry in results[::-1]:
        md_file.write(f'| {entry["program"]} | {entry["time_sec"]:.6f} sec |\n')

    md_file.write('\n![Performance Chart](benchmark.png)\n')

# Plot the performance comparison
plt.figure(figsize=(10, 5))
plt.barh(programs, times, color='darkblue')
plt.xlabel('Time (seconds)')
plt.ylabel('Program')
plt.title('Dictionary Performances')
plt.grid(axis='x', linestyle='--', alpha=0.7)

# Save and show the plot
plt.tight_layout()
plt.savefig('./results/benchmarks.png')

if not args.no_show:
    plt.show()