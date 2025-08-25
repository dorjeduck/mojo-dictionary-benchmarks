# Benchmarks

| Program                    | Time (seconds)          |
|----------------------------|-------------------------|
| compact_dict_fx.rs         | 0.075556 ± 0.005110 sec |
| compact_dict_ahash.rs      | 0.079252 ± 0.006470 sec |
| compact_dict.mojo          | 0.083276 ± 0.007969 sec |
| compact_dict_mojo_ahash.rs | 0.084626 ± 0.014298 sec |L
| a_hashmap.rs               | 0.145564 ± 0.007950 sec |
| hashmap.rs                 | 0.179874 ± 0.012438 sec |
| stdlib_dict.mojo           | 0.221199 ± 0.014452 sec |
| stdlib_dict.py             | 0.242619 ± 0.023634 sec |
| hashmap.rs (not optimized) | 0.399768 ± 0.011307 sec |
| python_dict.mojo           | 0.744607 ± 0.029020 sec |

![Performance Chart](benchmarks.png)
