//! Benchmark tool for measuring proof generation performance
//!
//! This tool measures proof generation time for different checkers
//! to establish baseline performance and compare with Ed25519 verification.

use clap::Parser;
use fuse_core::{zkvm, ProverType};
use serde_json::json;
use std::fs;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "fuse-benchmark")]
#[command(about = "Benchmark proof generation performance")]
struct Args {
    /// Path to compliance specification file
    #[arg(short, long)]
    spec: String,

    /// Path to system data file
    #[arg(short, long)]
    system: String,

    /// Number of iterations to run
    #[arg(short, long, default_value = "3")]
    iterations: u32,

    /// Checker type: "baseline" or "ed25519"
    #[arg(short, long, default_value = "baseline")]
    checker: String,

    /// Output results as JSON
    #[arg(short, long)]
    json: bool,
}

struct BenchmarkResult {
    iteration: u32,
    duration_secs: f64,
    success: bool,
    error: Option<String>,
}

struct BenchmarkSummary {
    checker: String,
    iterations: u32,
    results: Vec<BenchmarkResult>,
    average_secs: f64,
    min_secs: f64,
    max_secs: f64,
    first_run_secs: f64,
    subsequent_avg_secs: f64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("FUSE Benchmark Tool");
    println!("===================");
    println!("Checker: {}", args.checker);
    println!("Spec: {}", args.spec);
    println!("System: {}", args.system);
    println!("Iterations: {}", args.iterations);
    println!();

    // Read spec and system data
    let spec_json = fs::read_to_string(&args.spec)
        .map_err(|e| anyhow::anyhow!("Failed to read spec file {}: {}", args.spec, e))?;
    let system_json = fs::read_to_string(&args.system)
        .map_err(|e| anyhow::anyhow!("Failed to read system file {}: {}", args.system, e))?;

    // Run benchmarks
    let mut results = Vec::new();

    for i in 1..=args.iterations {
        println!("Running iteration {}/{}...", i, args.iterations);
        let start = Instant::now();

        match zkvm::generate_proof(&spec_json, &system_json, ProverType::Local) {
            Ok((_receipt_bytes, result, _journal_bytes)) => {
                let duration = start.elapsed();
                let duration_secs = duration.as_secs_f64();

                println!(
                    "  ✓ Completed in {:.2} seconds ({} minutes {:.2} seconds)",
                    duration_secs,
                    duration.as_secs() / 60,
                    duration.as_secs_f64() % 60.0
                );
                println!("  Result: {:?}", result);

                results.push(BenchmarkResult {
                    iteration: i,
                    duration_secs,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                let duration = start.elapsed();
                let duration_secs = duration.as_secs_f64();

                println!("  ✗ Failed after {:.2} seconds", duration_secs);
                println!("  Error: {}", e);

                results.push(BenchmarkResult {
                    iteration: i,
                    duration_secs,
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
        println!();
    }

    // Calculate summary
    let successful_results: Vec<&BenchmarkResult> = results
        .iter()
        .filter(|r| r.success)
        .collect();

    if successful_results.is_empty() {
        eprintln!("All iterations failed!");
        std::process::exit(1);
    }

    let durations: Vec<f64> = successful_results.iter().map(|r| r.duration_secs).collect();
    let average_secs = durations.iter().sum::<f64>() / durations.len() as f64;
    let min_secs = durations.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_secs = durations.iter().fold(0.0_f64, |a, &b| a.max(b));

    let first_run_secs = results[0].duration_secs;
    let subsequent_avg_secs = if results.len() > 1 {
        durations[1..].iter().sum::<f64>() / (durations.len() - 1) as f64
    } else {
        first_run_secs
    };

    let summary = BenchmarkSummary {
        checker: args.checker.clone(),
        iterations: args.iterations,
        results,
        average_secs,
        min_secs,
        max_secs,
        first_run_secs,
        subsequent_avg_secs,
    };

    // Output results
    if args.json {
        println!("{}", serde_json::to_string_pretty(&json!({
            "checker": summary.checker,
            "iterations": summary.iterations,
            "average_secs": summary.average_secs,
            "average_minutes": summary.average_secs / 60.0,
            "min_secs": summary.min_secs,
            "min_minutes": summary.min_secs / 60.0,
            "max_secs": summary.max_secs,
            "max_minutes": summary.max_secs / 60.0,
            "first_run_secs": summary.first_run_secs,
            "first_run_minutes": summary.first_run_secs / 60.0,
            "subsequent_avg_secs": summary.subsequent_avg_secs,
            "subsequent_avg_minutes": summary.subsequent_avg_secs / 60.0,
            "results": summary.results.iter().map(|r| json!({
                "iteration": r.iteration,
                "duration_secs": r.duration_secs,
                "duration_minutes": r.duration_secs / 60.0,
                "success": r.success,
                "error": r.error
            })).collect::<Vec<_>>()
        }))?);
    } else {
        println!("Benchmark Summary");
        println!("=================");
        println!("Checker: {}", summary.checker);
        println!("Iterations: {}", summary.iterations);
        println!();
        println!("Average: {:.2} seconds ({:.2} minutes)", summary.average_secs, summary.average_secs / 60.0);
        println!("Min:     {:.2} seconds ({:.2} minutes)", summary.min_secs, summary.min_secs / 60.0);
        println!("Max:     {:.2} seconds ({:.2} minutes)", summary.max_secs, summary.max_secs / 60.0);
        println!();
        println!("First run:     {:.2} seconds ({:.2} minutes)", summary.first_run_secs, summary.first_run_secs / 60.0);
        println!("Subsequent avg: {:.2} seconds ({:.2} minutes)", summary.subsequent_avg_secs, summary.subsequent_avg_secs / 60.0);
    }

    Ok(())
}

