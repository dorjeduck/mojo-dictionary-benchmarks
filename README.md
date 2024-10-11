# mojo-dictionary-benchmarks

This repository contains a set of programs designed to benchmark various dictionary implementations in **Mojo**, with additional comparisons to other programming languages (Python and Rust right now). The goal is to evaluate the performance of Mojo's dictionary implementations and provide a setup for experiments.

## Overview

The programs benchmark the following operations:

1. **Dictionary Initialization**: Creating and populating a dictionary-like data structure with a large number of items.
2. **Value Modification**: Modifying values for keys in the dictionary.
3. **Summation**: Calculating a sum based on the dictionary's values.

```python
...
for i in range(NUM):
    dic[str(i*2)] = i % 7
for i in range(0,NUM,2):
    dic[str(i*2)] *= 2
var sum_val = 0
for i in range(NUM):
    sum_val += dic[str(i*2)]
...
```

## Programs Benchmarked

- **Mojo**: (with Mojo 24.5 and current nightly build)
  - `stdlib_dict.mojo`: Uses Mojo's standard dictionary.
  - `compact_dict.mojo`: Uses [compact-dict](https://github.com/mzaks/compact-dict) for better memory efficiency.
  - `python_dict.mojo`: Uses Python's dictionary via Mojo's [Python integration](https://docs.modular.com/mojo/manual/python/).
  

- **Python**:
  - `stdlib_dict.py`: Implements the same operations as Mojo using Pythons's standard dictionary.

- **Rust**: 
  - `hashmap.rs`: Benchmarks Rust's `HashMap`, both in standard mode and with `-C opt-level=3` for optimization.

## Requirements

- Ensure that the `Magic` command line tool is installed by following the [Modular Docs](https://docs.modular.com/magic).
- Install Rust by following the instructions here: <https://www.rust-lang.org/tools/install>

## Running the Benchmarks

To run the benchmarks, use the provided shell script:

```sh
bash ./benchmarks.sh
```

## Benchmarks

| Program | Time (seconds) |
|---------|----------------|
| hashmap.rs (optimized) | 0.058493 sec |
| stdlib_dict.py | 0.373382 sec |
| hashmap.rs | 0.473159 sec |
| compact_dict.mojo (nightly) | 0.685255 sec |
| compact_dict.mojo | 0.697470 sec |
| stdlib_dict.mojo (nightly) | 2.438702 sec |
| stdlib_dict.mojo | 2.707037 sec |
| python_dict.mojo (nightly) | 10.089265 sec |
| python_dict.mojo | 10.785411 sec |

![Chart](./results/benchmarks.png)

## Contributing

Feel free to open issues or submit pull requests if you'd like to add more benchmarks, improve the current ones, or any other suggestions.

## Changelog

- 2024.10.11
  - Intial commit

## License

This project is licensed under the MIT License.
