use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub runs: usize,
    pub average: Duration,
    pub min: Duration,
    pub max: Duration,
    pub std_dev: Duration,
    pub durations: Vec<Duration>,
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("┌─────────────────────────────────────────");
        println!("│ Benchmark: {}", self.name);
        println!("├─────────────────────────────────────────");
        println!("│ Runs:    {}", self.runs);
        println!("│ Average: {:?}", self.average);
        println!("│ Min:     {:?}", self.min);
        println!("│ Max:     {:?}", self.max);
        println!("│ Std Dev: {:?}", self.std_dev);
        println!("└─────────────────────────────────────────");
    }
}

pub struct Benchmark {
    name: String,
    runs: usize,
    warmup_runs: usize,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            runs: 5,
            warmup_runs: 0,
        }
    }

    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs.max(1);
        self
    }

    pub fn warmup(mut self, warmup_runs: usize) -> Self {
        self.warmup_runs = warmup_runs;
        self
    }

    pub fn run<F, E>(self, f: F) -> Result<BenchmarkResult, E>
    where
        F: Fn() -> Result<(), E>,
    {
        for i in 0..self.warmup_runs {
            println!("Warmup run {} of {}...", i + 1, self.warmup_runs);
            f()?;
        }

        let mut durations = Vec::with_capacity(self.runs);

        for i in 0..self.runs {
            println!("Benchmark run {} of {}...", i + 1, self.runs);
            let start = Instant::now();
            f()?;
            let elapsed = start.elapsed();
            durations.push(elapsed);
        }

        let total: Duration = durations.iter().sum();
        let average = total / durations.len() as u32;
        let min = *durations.iter().min().unwrap();
        let max = *durations.iter().max().unwrap();
        let std_dev = calculate_std_dev(&durations, average);

        Ok(BenchmarkResult {
            name: self.name,
            runs: self.runs,
            average,
            min,
            max,
            std_dev,
            durations,
        })
    }
}

fn calculate_std_dev(durations: &[Duration], average: Duration) -> Duration {
    if durations.len() <= 1 {
        return Duration::ZERO;
    }

    let avg_nanos = average.as_nanos() as f64;
    let variance: f64 = durations
        .iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - avg_nanos;
            diff * diff
        })
        .sum::<f64>()
        / (durations.len() - 1) as f64;

    Duration::from_nanos(variance.sqrt() as u64)
}

#[derive(Debug)]
pub struct SuiteResult {
    pub name: String,
    pub results: Vec<BenchmarkResult>,
}

impl SuiteResult {
    pub fn print_summary(&self) {
        println!("\n╔═════════════════════════════════════════════════════════════════");
        println!("║ Benchmark Suite: {}", self.name);
        println!("╠═════════════════════════════════════════════════════════════════");

        for result in &self.results {
            println!("║");
            println!("║ 📊 {}", result.name);
            println!(
                "║    Runs: {} | Avg: {:?} | Min: {:?} | Max: {:?}",
                result.runs, result.average, result.min, result.max
            );
        }

        println!("║");
        println!("╚═════════════════════════════════════════════════════════════════\n");
    }

    pub fn fastest(&self) -> Option<&BenchmarkResult> {
        self.results.iter().min_by_key(|r| r.average)
    }

    pub fn slowest(&self) -> Option<&BenchmarkResult> {
        self.results.iter().max_by_key(|r| r.average)
    }
}

pub struct BenchmarkSuite {
    name: String,
    runs: usize,
    benchmarks: Vec<(
        String,
        Box<dyn Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>>>,
    )>,
}

impl BenchmarkSuite {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            runs: 5,
            benchmarks: Vec::new(),
        }
    }

    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs.max(1);
        self
    }

    pub fn add<F>(mut self, name: &str, f: F) -> Self
    where
        F: Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>> + 'static,
    {
        self.benchmarks.push((name.to_string(), Box::new(f)));
        self
    }

    pub fn run(self) -> Result<SuiteResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        for (name, func) in self.benchmarks {
            let result = Benchmark::new(&name).runs(self.runs).run(func)?;
            results.push(result);
        }

        Ok(SuiteResult {
            name: self.name,
            results,
        })
    }
}
