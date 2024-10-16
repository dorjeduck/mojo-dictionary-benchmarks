# Mojo dictionary benchmarks

This repository contains programs to benchmark different dictionary implementations in [Mojo](https://www.modular.com/mojo), with comparisons to other languages (currently Python and Rust). The main goal is to evaluate the performance of Mojo's dictionary implementations and set up a simple framework for future evaluations.

## Overview

The programs benchmark the following operations:

1. **Dictionary Initialization**: Creating and populating a dictionary-like data structure with a large number of items.
2. **Value Modification**: Modifying values for keys in the dictionary.
3. **Summation**: Calculating a sum based on the dictionary's values.

```python
...
var keys = List[String](capacity = NUM)
for i in range(NUM):
    keys[i] = "k"+str(i) 

var start = now()

var dic = Dict[String,Int]()
for i in range(NUM):
    dic[keys[i]] = i % 7
for i in range(0,NUM,2):
    dic[keys[i]] *= 2
var sum_val = 0
for i in range(NUM):
    sum_val += dic[keys[i]]

var elapsed = (now()-start)/1e9 
...
```

## Programs benchmarked

- **Mojo**: (compiled with Mojo 24.5 and current nightly build)
  - `stdlib_dict.mojo`: Uses Mojo's standard dictionary.
  - `compact_dict.mojo`: Uses [compact-dict](https://github.com/mzaks/compact-dict).
  - `python_dict.mojo`: Uses Python's dictionary via Mojo's [Python integration](https://docs.modular.com/mojo/manual/python/).
  
- **Python**:
  - `stdlib_dict.py`: Implements the same operations as in Mojo using Pythons's standard dictionary.

- **Rust**:
  - `hashmap.rs`: Benchmarks Rust's `HashMap`, both in standard mode and with `-C opt-level=3` for optimization. (Suggestions on how to improve the performance of this basic implementation most welcome.)

## Requirements

- Install the `Magic` command line tool by following the instructions in the [Modular Documentaion](https://docs.modular.com/magic).
- Ensure that both Rust and Python are installed on your system.

## Running the Benchmarks

To run the benchmarks, use the provided shell script:

```sh
bash ./benchmarks.sh
```

After running the benchmarks, you can view the performance comparison in `results/benchmarks.md` and a plot in `results/benchmark.png`.

## Benchmarks

| Program | Time (seconds) |
|---------|----------------|
| compact_dict.mojo (nightly) | 0.082251 sec |
| compact_dict.mojo | 0.084032 sec |
| hashmap.rs (optimized) | 0.143373 sec |
| stdlib_dict.py | 0.264387 sec |
| hashmap.rs | 0.470244 sec |
| stdlib_dict.mojo (nightly) | 2.319244 sec |
| stdlib_dict.mojo | 2.523637 sec |
| python_dict.mojo (nightly) | 9.789734 sec |
| python_dict.mojo | 10.352229 sec |

![Chart](./results/benchmarks.png)

## Contributing

Contributions are welcome! If you'd like to add new benchmarks, enhance the current ones, or any suggestions, please open an issue or submit a pull request.

## Changelog

- 2024.10.12
  - Reimplemented based on feedback provided by the community on the [Modular Discord server](https://discord.gg/xZktyT2q).
- 2024.10.11
  - Intial commit

## License

This project is licensed under the MIT License.
