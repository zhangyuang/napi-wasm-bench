# performance-demo

```bash
npm install
npm run build
npm run benchmark
```

## Result

```bash
$ node benchmark.js
Running "String Processing Benchmark" suite...
Progress: 100%

  WASM Implementation:
    17 136 ops/s, ±4.47%   | slowest, 43.56% slower

  N-API Implementation:
    30 362 ops/s, ±1.15%   | fastest

Finished 2 cases!
  Fastest: N-API Implementation
  Slowest: WASM Implementation
```