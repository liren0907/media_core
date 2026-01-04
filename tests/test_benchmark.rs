use media_core::benchmark::{Benchmark, Report};
use std::time::Duration;
use tempfile::tempdir;

/// Example: Single function benchmark (matches bin/benchmark.rs usage)
#[test]
fn test_benchmark_single_function() {
    println!("=== Test: Benchmark Single Function ===");

    // Simple function to benchmark (fast operation)
    let result = Benchmark::new("Vector Sum").runs(3).run(|| {
        let _sum: i32 = (0..1000).sum();
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    });

    assert!(result.is_ok(), "Benchmark failed: {:?}", result.err());

    let bench_result = result.unwrap();
    assert_eq!(bench_result.name, "Vector Sum");
    assert_eq!(bench_result.runs, 3);
    assert_eq!(bench_result.durations.len(), 3);
    assert!(bench_result.average > Duration::ZERO);
    assert!(bench_result.min <= bench_result.average);
    assert!(bench_result.max >= bench_result.average);

    println!("✅ Benchmark completed: {} runs", bench_result.runs);
    println!("   Average: {:?}", bench_result.average);
    println!("=== Test Passed ===\n");
}

/// Test BenchmarkResult.print_summary() (matches bin/benchmark.rs)
#[test]
fn test_benchmark_result_summary() {
    println!("=== Test: BenchmarkResult Summary ===");

    let result = Benchmark::new("Print Summary Test")
        .runs(2)
        .run(|| {
            std::thread::sleep(Duration::from_millis(1));
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .expect("Benchmark failed");

    // This should not panic
    result.print_summary();

    println!("✅ print_summary() executed successfully");
    println!("=== Test Passed ===\n");
}

/// Test Report.to_json() for BenchmarkResult
#[test]
fn test_benchmark_result_to_json() {
    println!("=== Test: BenchmarkResult to JSON ===");

    let output_dir = tempdir().expect("Failed to create temp dir");
    let json_path = output_dir.path().join("benchmark.json");

    let result = Benchmark::new("JSON Export Test")
        .runs(2)
        .run(|| {
            let _: i32 = (0..100).sum();
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .expect("Benchmark failed");

    let export_result = result.to_json(json_path.to_str().unwrap());
    assert!(
        export_result.is_ok(),
        "JSON export failed: {:?}",
        export_result.err()
    );
    assert!(json_path.exists());

    // Verify JSON content
    let content = std::fs::read_to_string(&json_path).unwrap();
    assert!(content.contains("\"name\": \"JSON Export Test\""));
    assert!(content.contains("\"runs\": 2"));
    assert!(content.contains("\"average_ms\""));

    println!("✅ JSON export successful");
    println!("=== Test Passed ===\n");
}
