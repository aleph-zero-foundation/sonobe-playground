# sonobe-playground

## Current state

- ✅ Nova folding
- ✅ HyperNova folding without multifolding (μ = ν = 0)
- ✅ HyperNova multifolding (μ = ν = 1)

## Reproduce

```bash
make prepare-circuit
make run
```

### Expected output

_12th Gen Intel® Core™ i7-12800H × 20, 32Gb RAM_

```
Prepare circuit: 386.733µs
Prepare input: 9.041434ms
========== Nova folding scheme ====================
Prepare folding: 1.042210402s
Transform input: 352.973µs
Prove_step 0: 188.311556ms
Prove_step 1: 201.33003ms
Prove_step 2: 256.05521ms
Prove_step 3: 282.359763ms
Prove_step 4: 252.831315ms
Prove_step 5: 255.913864ms
Folding verification: 16.793244ms
========== HyperNova<1,1> folding scheme ==========
Prepare folding: 2.065175679s
Transform input: 212.792µs
Prove_step 0: 796.910579ms
Prove_step 1: 853.848474ms
Prove_step 2: 930.1784ms
Prove_step 3: 872.190521ms
Prove_step 4: 910.903061ms
Prove_step 5: 939.589533ms
Folding verification: 29.238423ms
========== HyperNova<2,2> folding scheme ==========
Prepare folding: 2.995732852s
Transform input: 3.9520013s
Prove_step 0: 966.629701ms
Prove_step 1: 1.22535041s
Folding verification: 33.430805ms
```
