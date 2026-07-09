use std::process::{Command, Stdio};
use std::time::Instant;
use serde::Serialize;

#[derive(Serialize)]
struct BenchReport {
    runs: usize,
    baseline_ms: Vec<u128>,
    hyper_ms: Vec<u128>,
    baseline_median: u128,
    hyper_median: u128,
    improvement_ms: i128,
    improvement_pct: f64,
}

fn main() -> anyhow::Result<()> {
    ensure_daemon()?;

    let runs = 5usize;
    let mut base = Vec::new();
    let mut hyp = Vec::new();

    for _ in 0..runs {
        base.push(run(&["build"])?);
        hyp.push(run_hyper(&["build"])?);
    }

    let bmed = median(&mut base.clone());
    let hmed = median(&mut hyp.clone());
    let improvement_ms = bmed as i128 - hmed as i128;
    let improvement_pct = if bmed > 0 { (improvement_ms as f64) * 100.0 / (bmed as f64) } else { 0.0 };

    println!("baseline_median_ms={}", bmed);
    println!("hyper_median_ms={}", hmed);
    println!("improvement_ms={}", improvement_ms);
    println!("improvement_pct={:.2}", improvement_pct);

    let report = BenchReport {
        runs, baseline_ms: base, hyper_ms: hyp,
        baseline_median: bmed, hyper_median: hmed,
        improvement_ms, improvement_pct
    };
    std::fs::write("hyperc.bench.json", serde_json::to_string_pretty(&report)?)?;
    println!("wrote hyperc.bench.json");
    Ok(())
}

fn ensure_daemon() -> anyhow::Result<()> {
    let out = Command::new("cargo")
        .args(["hyper","daemon","status"])
        .stdout(Stdio::piped()).stderr(Stdio::piped())
        .output();

    let running = out.ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.contains("\"status\":\"running\""))
        .unwrap_or(false);

    if !running {
        let _ = Command::new("cargo").args(["hyper","daemon","start"])
            .stdout(Stdio::null()).stderr(Stdio::null()).status();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    Ok(())
}

fn median(v: &mut Vec<u128>) -> u128 {
    v.sort_unstable();
    v[v.len()/2]
}

fn run(args: &[&str]) -> anyhow::Result<u128> {
    let start = Instant::now();
    let status = Command::new("cargo").args(args).stdout(Stdio::null()).stderr(Stdio::null()).status()?;
    if !status.success() { anyhow::bail!("baseline cargo failed"); }
    Ok(start.elapsed().as_millis())
}

fn run_hyper(args: &[&str]) -> anyhow::Result<u128> {
    let start = Instant::now();
    let status = Command::new("cargo").args(["hyper"].into_iter().chain(args.iter().copied())).status()?;
    if !status.success() { anyhow::bail!("cargo hyper failed"); }
    Ok(start.elapsed().as_millis())
}
