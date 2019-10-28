# zkInterface HTTP servers and benchmark

| File | Description |
| - | - |
| `*-server` | HTTP server executables wrapping various proof systems. |
| `benchmark/src/main.rs` | Run various benchmarks and report average runtimes. |
| `benchmark/src/circuit.rs` | Generate test circuits of different sizes. |
| `benchmark/src/runner.rs` | Request proofs from the servers with an HTTP client. |

## Run the benchmark

```
cd benchmark
cargo bench
```
