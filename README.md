# pinscher

A small benchmarking crate.

## Usage

Add this to your `Cargo.toml` file:

```
[dependencies]
pinscher = { git = "https://github.com/jeandiegof/pinscher" } 
```

And then:

```Rust
use pinscher::{BenchSuite, CpuTimeBencher, EnergyBencher};

fn main() {
    // Choose your Bencher
    let mut cpu_time_bencher = CpuTimeBencher::new();
    BenchSuite::bench(|| target_function(), &mut cpu_time_bencher).unwrap();
    println!("{}", cpu_time_bencher.cpu_time().unwrap().as_micros());
}
```

## Benchers available

- **CpuTime**: measure the active CPU taken by the function.
- **Energy**: uses Intel RAPL to measure the energy used during the execution of the function.
