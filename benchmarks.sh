#!/bin/bash

# Ensure directories exists
mkdir -p bin
mkdir -p results

# Run Mojo scripts
echo "mojo stdlib_dict.mojo"
magic run mojo build -o ./bin/stdlib_dict stdlib_dict.mojo && ./bin/stdlib_dict
echo "mojo stdlib_dict.mojo (nightly)"
magic run -e nightly mojo build -o ./bin/stdlib_dict_nightly stdlib_dict.mojo && ./bin/stdlib_dict_nightly --nightly
echo "mojo compact_dict.mojo"
magic run mojo build -o ./bin/compact_dict compact_dict.mojo && ./bin/compact_dict
echo "mojo compact_dict.mojo (nightly)"
magic run -e nightly mojo build -o ./bin/compact_dict_nightly compact_dict.mojo && ./bin/compact_dict_nightly --nightly
echo "mojo python_dict.mojo"
magic run mojo build -o ./bin/python_dict python_dict.mojo && ./bin/python_dict
echo "mojo python_dict.mojo (nightly)"
magic run -e nightly mojo build -o ./bin/python_dict_nightly python_dict.mojo && ./bin/python_dict_nightly --nightly


# Run Python script
echo "python stdlib_dict.py"
python stdlib_dict.py

# Run Rust program without optimization
echo "rustc hashmap.rs"
rustc hashmap.rs -o ./bin/hashmap && ./bin/hashmap

# Run Rust program with optimization
echo "rustc hashmap.rs -C opt-level=3"
rustc hashmap.rs -C opt-level=3 -o ./bin/hashmap_optimized && ./bin/hashmap_optimized --optimized

# Plot results
echo ""
python plot.py
