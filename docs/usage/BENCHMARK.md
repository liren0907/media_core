# Benchmark Module

Performance measurement utilities for timing and comparing operations.

**Location:** `src/benchmark/`

## Module Structure

```
benchmark/
├── mod.rs      # Exports
├── runner.rs   # Benchmark, BenchmarkResult, BenchmarkSuite, SuiteResult
└── report.rs   # Report trait (to_console, to_json)
```

## Quick Start

### Single Benchmark

```rust
use media_core::benchmark::Benchmark;

let result = Benchmark::new("My Operation")
    .runs(5)           // Number of timed runs (default: 5)
    .warmup(1)         // Optional warmup runs
    .run(|| {
        // Your code to benchmark
        expensive_operation()?;
        Ok(())
    })?;

result.print_summary();
```

### Export to JSON

```rust
use media_core::benchmark::{Benchmark, Report};

let result = Benchmark::new("My Operation")
    .runs(3)
    .run(|| { /* ... */ Ok(()) })?;

result.to_json("benchmark_results.json")?;
```

---

## Types

### `Benchmark`

Builder for creating and running benchmarks.

| Method | Description |
|--------|-------------|
| `new(name)` | Create a new benchmark |
| `runs(n)` | Set number of timed runs (default: 5) |
| `warmup(n)` | Set warmup runs before timing |
| `run(closure)` | Execute and return `BenchmarkResult` |

### `BenchmarkResult`

Results from a benchmark run.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Benchmark name |
| `runs` | `usize` | Number of runs |
| `average` | `Duration` | Average duration |
| `min` | `Duration` | Fastest run |
| `max` | `Duration` | Slowest run |
| `std_dev` | `Duration` | Standard deviation |
| `durations` | `Vec<Duration>` | All run durations |

**Methods:**
- `print_summary()` - Print formatted results to console

### `Report` Trait

```rust
pub trait Report {
    fn to_console(&self);
    fn to_json(&self, path: &str) -> Result<(), std::io::Error>;
}
```

Implemented for `BenchmarkResult` and `SuiteResult`.

---

## Advanced: Benchmark Suite

Run multiple benchmarks and compare results:

```rust
use media_core::benchmark::BenchmarkSuite;

let suite = BenchmarkSuite::new("Comparison Suite")
    .runs(3)
    .add("Method A", || { method_a()?; Ok(()) })
    .add("Method B", || { method_b()?; Ok(()) })
    .run()?;

suite.print_summary();
println!("Fastest: {:?}", suite.fastest());
```

---

## Feature Overview

| Component | Feature | Defined In |
|-----------|---------|------------|
| **`Benchmark`** | Single function timing | `runner.rs` |
| **`BenchmarkResult`** | Timing statistics (avg, min, max, std_dev) | `runner.rs` |
| **`BenchmarkSuite`** | Run and compare multiple benchmarks | `runner.rs` |
| **`Report`** | Export to console or JSON | `report.rs` |
