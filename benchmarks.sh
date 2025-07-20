#!/bin/bash

# Ensure directories exists
mkdir -p bin
mkdir -p results

# Run Mojo scripts
echo "mojo stdlib_dict.mojo"
pixi run mojo build -o ./bin/stdlib_dict stdlib_dict.mojo && ./bin/stdlib_dict
echo "mojo compact_dict.mojo"
pixi run mojo build -o ./bin/compact_dict compact_dict.mojo && ./bin/compact_dict
echo "mojo python_dict.mojo"
pixi run mojo build -o ./bin/python_dict python_dict.mojo && ./bin/python_dict


# Run Python script
echo "python stdlib_dict.py"
python stdlib_dict.py

# Run Rust program without optimization
echo "rustc hashmap.rs"
rustc hashmap.rs -C target-cpu=native -o ./bin/hashmap && ./bin/hashmap

# Run Rust program with optimization
echo "rustc hashmap.rs -C opt-level=3"
rustc hashmap.rs -C target-cpu=native -C opt-level=3 -o ./bin/hashmap_optimized && ./bin/hashmap_optimized --optimized

# Plot results
echo ""
python plot.py
