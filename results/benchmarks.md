# Benchmarks

| Program | Time (seconds) |
|---------|----------------|
| -C opt-level=3 hashmap.rs | 0.099639 sec |
| stdlib_dict.py | 0.395997 sec |
| hashmap.rs | 0.469715 sec |
| compact_dict.mojo (nightly) | 0.680922 sec |
| compact_dict.mojo | 0.752529 sec |
| stdlib_dict.mojo (nightly) | 2.549491 sec |
| stdlib_dict.mojo | 2.757368 sec |
| python_dict.mojo (nightly) | 10.356744 sec |
| python_dict.mojo | 10.984972 sec |

![Performance Chart](benchmark.png)
