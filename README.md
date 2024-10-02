# sonobe-playground

## Current state

- ✅ Nova folding
- ✅ HyperNova folding without multifolding (μ = ν = 0)
- ❌ HyperNova multifolding (μ = ν = 1)

## Reproduce

```bash
make prepare-circuit
make run
```

### Expected output

_12th Gen Intel® Core™ i7-12800H × 20, 32Gb RAM_

```
Prepare circuit: 62.569µs
Prepare input: 12.271114ms
========== Nova folding scheme ====================
Prepare folding: 1.038241793s
Transform input: 293.007µs
Prove_step 0: 186.297898ms
Prove_step 1: 216.137291ms
Prove_step 2: 244.387123ms
Prove_step 3: 253.724876ms
Prove_step 4: 249.476304ms
Prove_step 5: 254.825831ms
Folding verification: 15.513019ms
========== HyperNova<1,1> folding scheme ==========
Prepare folding: 2.125681209s
Transform input: 221.36µs
Prove_step 0: 765.455805ms
Prove_step 1: 848.616158ms
Prove_step 2: 833.972256ms
Prove_step 3: 864.703408ms
Prove_step 4: 844.404617ms
Prove_step 5: 865.9787ms
Folding verification: 31.417487ms
========== HyperNova<2,2> folding scheme ==========
Prepare folding: 2.843266713s
Transform input: 3.922030432s
Prove_step 0: 995.358483ms
Prove_step 1: 1.209482586s
thread 'main' panicked at src/folding.rs:159:6:
Failed to verify folded proof: IVCVerificationFail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
make: *** [Makefile:17: run] Error 101
```
