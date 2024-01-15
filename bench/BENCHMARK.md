| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target\release\fakelake.exe generate bench\fakelake_input.yaml` | 252.8 ± 3.3 | 249.0 | 260.0 | 1.00 |
| `python bench\mimesis_bench.py` | 3374.9 ± 21.3 | 3353.0 | 3426.2 | 13.35 ± 0.19 |
| `python bench\faker_bench.py` | 13552.7 ± 340.5 | 13336.4 | 14446.4 | 53.62 ± 1.52 |
