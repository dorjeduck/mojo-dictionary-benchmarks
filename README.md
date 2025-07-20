# Mojo dictionary benchmarks

This repository contains programs to benchmark different dictionary implementations
in [Mojo](https://www.modular.com/mojo), with comparisons to other languages (currently Python and Rust). The main goal
is to evaluate the performance of Mojo's dictionary implementations and set up a simple framework for future
evaluations.

## Overview

The programs benchmark the following operations:

1. **Dictionary Initialization**: Creating and populating a dictionary-like data structure with a large number of items.
2. **Value Modification**: Modifying values for keys in the dictionary.
3. **Summation**: Calculating a sum based on the dictionary's values.

```mojo
...
var keys = List[String](capacity=NUM)
for i in range(NUM):
    keys[i] = "k" + str(i)

var start = now()

var dic = Dict[String, Int]()
for i in range(NUM):
    dic[keys[i]] = i % 7
for i in range(0, NUM, 2):
    dic[keys[i]] *= 2
var sum_val = 0
for i in range(NUM):
    sum_val += dic[keys[i]]

var elapsed = (now() - start) / 1e9
...
```

## Programs benchmarked

- **Mojo**: (compiled with Mojo 24.5 and 25.5)
    - `stdlib_dict.mojo`: Uses Mojo's standard dictionary.
    - `compact_dict.mojo`: Uses [compact-dict](https://github.com/mzaks/compact-dict).
    - `python_dict.mojo`: Uses Python's dictionary via Mojo
      s [Python integration](https://docs.modular.com/mojo/manual/python/).

- **Python**:
    - `stdlib_dict.py`: Implements the same operations as in Mojo using Python's standard dictionary.

- **Rust**:
    - `hashmap.rs`: Benchmarks Rust's `HashMap`, both in standard mode and with `-C opt-level=3` for optimization. 
      (Suggestions on how to improve the performance of this basic implementation are most welcome.)

## Requirements

- Install the `pixi` command line tool by following the instructions in
  the [Mojo Documentation](https://docs.modular.com/mojo/manual/get-started).
- Ensure that both Rust and Python are installed on your system.

## Running the Benchmarks

To run the benchmarks, use the provided shell script:

```sh
bash ./benchmarks.sh
```

After running the benchmarks, you can view the performance comparison in `results/benchmarks.md` and a plot in
`results/benchmark.png`.

## Benchmarks

| Benchmark              | Mojo 24.5     | Mojo 25.5      |
|------------------------|---------------|----------------|
| Measured at            | 11 Oct 2024   | 19 Jul 2025    |  
| ---------------------  | ------------- | -------------- |
| compact_dict.mojo      | 0.084032 sec  | 0.079602 sec   |
| hashmap.rs (optimized) | 0.143373 sec  | 0.174457 sec   |
| stdlib_dict.py         | 0.264387 sec  | 0.233545 sec   |
| hashmap.rs             | 0.470244 sec  | 0.398161 sec   |
| stdlib_dict.mojo       | 2.523637 sec  | 0.214174 sec   |
| python_dict.mojo       | 10.352229 sec | 0.958389 sec   |

![Chart](./results/benchmarks.png)

## Contributing

Contributions are welcome! If you'd like to add new benchmarks, enhance the current ones, or any suggestions, please
open an issue or submit a pull request.

## Changelog

- 2025.07.19
    - Code updated to reflect changes in [mojo spec](https://docs.modular.com/mojo/changelog/) 
- 2024.10.12
    - Reimplemented based on feedback provided by the community on
      the [Modular Discord server](https://discord.gg/xZktyT2q).
- 2024.10.11
    - Initial commit

## License

This project is licensed under the MIT License.
