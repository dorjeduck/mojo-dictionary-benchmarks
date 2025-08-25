#!/bin/bash

# Ensure directories exists
mkdir -p bin
mkdir -p results

declare -A programs
programs=(
    ["mojo_stdlib"]="pixi run mojo build -o ./bin/stdlib_dict stdlib_dict.mojo && ./bin/stdlib_dict"
    ["mojo_compact"]="pixi run mojo build -o ./bin/compact_dict compact_dict.mojo && ./bin/compact_dict"
    ["mojo_python_dict"]="pixi run mojo build -o ./bin/python_dict python_dict.mojo && ./bin/python_dict"
    ["python_stdlib"]="python stdlib_dict.py"
    ["rustc_plain"]="rustc hashmap.rs -C target-cpu=native -o ./bin/hashmap && ./bin/hashmap"
    ["rustc_opt"]="rustc hashmap.rs -C target-cpu=native -C opt-level=3 -o ./bin/hashmap_optimized && ./bin/hashmap_optimized --optimized"
    ["cargo_a_hashmap"]="cargo run --release --bin a_hashmap"
    ["cargo_compact_fx"]="cargo run --release --bin compact_dict_fx"
    ["cargo_compact_ahash"]="cargo run --release --bin compact_dict_ahash"
    ["cargo_compact_mojo_ahash"]="cargo run --release --bin compact_dict_mojo_ahash"
)

# Define how many iterations for each program
declare -A loops
loops=(
    ["mojo_stdlib"]=30
    ["mojo_compact"]=30
    ["mojo_python_dict"]=20
    ["python_stdlib"]=30
    ["rustc_plain"]=20
    ["rustc_opt"]=20
    ["cargo_a_hashmap"]=30
    ["cargo_compact_fx"]=30
    ["cargo_compact_ahash"]=20
    ["cargo_compact_mojo_ahash"]=60
)

for prog in "${!programs[@]}"; do
    echo "== Running $prog =="
    for ((i=1; i<=${loops[$prog]}; i++)); do
        eval "${programs[$prog]}"
    done
done

# Plot results
echo ""
python aggregate_results.py
python plot.py
