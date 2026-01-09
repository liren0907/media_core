//! Benchmark Pipeline Steps
//!
//! This module provides pipeline steps for benchmarking operations.
//! Unlike other pipeline steps that transform data, benchmark steps
//! measure performance of operations within the pipeline context.

use crate::benchmark::runner::BenchmarkResult;
use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use std::time::{Duration, Instant};

// ============================================================================
// BENCHMARK PIPELINE RESULT TYPE
// ============================================================================

/// Result of benchmark operations stored in the pipeline context
#[derive(Debug, Clone, Default)]
pub struct BenchmarkPipelineResult {
    /// Individual benchmark results
    pub results: Vec<BenchmarkResult>,
    /// Total benchmarking time
    pub total_time: Duration,
}

impl BenchmarkPipelineResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            total_time: Duration::ZERO,
        }
    }

    /// Add a benchmark result
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }

    /// Print summary of all benchmark results
    pub fn print_summary(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════");
        println!("║ Pipeline Benchmark Results");
        println!("╠═══════════════════════════════════════════════════════════");

        for result in &self.results {
            println!("║");
            println!("║ 📊 {}", result.name);
            println!(
                "║    Runs: {} | Avg: {:?} | Min: {:?} | Max: {:?}",
                result.runs, result.average, result.min, result.max
            );
        }

        println!("║");
        println!("║ Total benchmarking time: {:?}", self.total_time);
        println!("╚═══════════════════════════════════════════════════════════\n");
    }
}

// ============================================================================
// BENCHMARK CONTEXT STEP
// ============================================================================

/// A pipeline step that benchmarks the context initialization time.
///
/// This step measures how long it takes to access the media source
/// and validates that the source is accessible.
pub struct BenchmarkContext {
    name: String,
    runs: usize,
}

impl BenchmarkContext {
    /// Create a new BenchmarkContext step
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            runs: 3,
        }
    }

    /// Set the number of benchmark runs
    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs.max(1);
        self
    }
}

impl PipelineStep<MediaContext> for BenchmarkContext {
    fn name(&self) -> &str {
        "BenchmarkContext"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let start = Instant::now();

        // Benchmark accessing the source path
        let mut durations = Vec::with_capacity(self.runs);

        for _ in 0..self.runs {
            let run_start = Instant::now();

            // Access context source
            let _ = context.source.as_path_str();

            durations.push(run_start.elapsed());
        }

        // Calculate statistics
        let total: Duration = durations.iter().sum();
        let average = total / durations.len() as u32;
        let min = *durations.iter().min().unwrap_or(&Duration::ZERO);
        let max = *durations.iter().max().unwrap_or(&Duration::ZERO);
        let std_dev = calculate_std_dev(&durations, average);

        let result = BenchmarkResult {
            name: self.name.clone(),
            runs: self.runs,
            average,
            min,
            max,
            std_dev,
            durations,
        };

        // Store in context
        let benchmark_result = context
            .benchmark_result
            .get_or_insert_with(BenchmarkPipelineResult::new);
        benchmark_result.add_result(result);
        benchmark_result.total_time += start.elapsed();

        Ok(())
    }

    fn is_critical(&self) -> bool {
        false // Benchmark failures should not stop the pipeline
    }
}

// ============================================================================
// BENCHMARK METADATA EXTRACTION STEP
// ============================================================================

/// A pipeline step that benchmarks metadata extraction.
pub struct BenchmarkMetadataExtraction {
    name: String,
    runs: usize,
    include_thumbnail: bool,
}

impl BenchmarkMetadataExtraction {
    /// Create a new BenchmarkMetadataExtraction step
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            runs: 3,
            include_thumbnail: false,
        }
    }

    /// Set the number of benchmark runs
    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs.max(1);
        self
    }

    /// Include thumbnail generation in the benchmark
    pub fn include_thumbnail(mut self, include: bool) -> Self {
        self.include_thumbnail = include;
        self
    }
}

impl PipelineStep<MediaContext> for BenchmarkMetadataExtraction {
    fn name(&self) -> &str {
        "BenchmarkMetadataExtraction"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        use crate::metadata::orchestrator::get_media_info;

        let path_str = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError(
                "BenchmarkMetadataExtraction requires a File source".to_string(),
            )
        })?;

        let start = Instant::now();
        let mut durations = Vec::with_capacity(self.runs);

        for i in 0..self.runs {
            println!("   Benchmark run {} of {}...", i + 1, self.runs);
            let run_start = Instant::now();

            // Run metadata extraction
            let _ = get_media_info(path_str, self.include_thumbnail);

            durations.push(run_start.elapsed());
        }

        // Calculate statistics
        let total: Duration = durations.iter().sum();
        let average = total / durations.len() as u32;
        let min = *durations.iter().min().unwrap_or(&Duration::ZERO);
        let max = *durations.iter().max().unwrap_or(&Duration::ZERO);
        let std_dev = calculate_std_dev(&durations, average);

        let result = BenchmarkResult {
            name: self.name.clone(),
            runs: self.runs,
            average,
            min,
            max,
            std_dev,
            durations,
        };

        // Store in context
        let benchmark_result = context
            .benchmark_result
            .get_or_insert_with(BenchmarkPipelineResult::new);
        benchmark_result.add_result(result);
        benchmark_result.total_time += start.elapsed();

        Ok(())
    }

    fn is_critical(&self) -> bool {
        false
    }
}

// ============================================================================
// BENCHMARK FRAME EXTRACTION STEP
// ============================================================================

/// A pipeline step that benchmarks frame extraction.
pub struct BenchmarkFrameExtraction {
    name: String,
    runs: usize,
    frame_count: usize,
}

impl BenchmarkFrameExtraction {
    /// Create a new BenchmarkFrameExtraction step
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            runs: 3,
            frame_count: 10,
        }
    }

    /// Set the number of benchmark runs
    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = runs.max(1);
        self
    }

    /// Set how many frames to extract per run
    pub fn frame_count(mut self, count: usize) -> Self {
        self.frame_count = count.max(1);
        self
    }
}

impl PipelineStep<MediaContext> for BenchmarkFrameExtraction {
    fn name(&self) -> &str {
        "BenchmarkFrameExtraction"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        use crate::streaming::{SamplingStrategy, StreamExtractor};

        let path_str = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError(
                "BenchmarkFrameExtraction requires a File source".to_string(),
            )
        })?;

        let start = Instant::now();
        let mut durations = Vec::with_capacity(self.runs);

        for i in 0..self.runs {
            println!("   Benchmark run {} of {}...", i + 1, self.runs);
            let run_start = Instant::now();

            // Run frame extraction
            let strategy = SamplingStrategy::FirstN(self.frame_count);
            let mut extractor = StreamExtractor::new(path_str, Some(strategy)).map_err(|e| {
                PipelineError::StepFailed {
                    step_name: self.name().to_string(),
                    error: e,
                }
            })?;

            let _ = extractor.extract(None);

            durations.push(run_start.elapsed());
        }

        // Calculate statistics
        let total: Duration = durations.iter().sum();
        let average = total / durations.len() as u32;
        let min = *durations.iter().min().unwrap_or(&Duration::ZERO);
        let max = *durations.iter().max().unwrap_or(&Duration::ZERO);
        let std_dev = calculate_std_dev(&durations, average);

        let result = BenchmarkResult {
            name: self.name.clone(),
            runs: self.runs,
            average,
            min,
            max,
            std_dev,
            durations,
        };

        // Store in context
        let benchmark_result = context
            .benchmark_result
            .get_or_insert_with(BenchmarkPipelineResult::new);
        benchmark_result.add_result(result);
        benchmark_result.total_time += start.elapsed();

        Ok(())
    }

    fn is_critical(&self) -> bool {
        false
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

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
