use clap::{Parser, Subcommand, Args};
use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::Stdio;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc};
use std::time::{Duration, Instant};
use std::thread;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name="hyperc-daemon", version, about="HyperC build daemon")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve(ServeArgs),
}

#[derive(Args)]
struct ServeArgs {
    /// Address to listen on (env: HYPERC_ADDR)
    #[arg(long, env="HYPERC_ADDR", default_value="127.0.0.1:7878")]
    addr: String,
    /// Directory to watch for Cargo.lock changes
    #[arg(long, default_value=".")]
    watch: String,
}

#[derive(Deserialize)]
struct BuildReq {
    kind: String,
    cwd: Option<String>,
    args: Option<Vec<String>>,
    cmd: Option<String>,
}

#[derive(Serialize)]
struct BuildResp {
    ok: bool,
    elapsed_ms: u128,
    stdout: String,
    stderr: String,
    sccache_pre: String,
    sccache_post: String,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").with_target(false).compact().init();
    match Cli::parse().cmd {
        Command::Serve(args) => serve(args),
    }
}

fn serve(args: ServeArgs) -> anyhow::Result<()> {
    info!("listening on {}", args.addr);
    let listener = TcpListener::bind(&args.addr)?;
    let stop = Arc::new(AtomicBool::new(false));

    // spawn watcher
    spawn_watcher(args.watch.clone());

    for stream in listener.incoming() {
        if stop.load(Ordering::SeqCst) { break; }
        match stream {
            Ok(mut s) => {
                let mut buf = String::new();
                s.read_to_string(&mut buf).ok();
                let req: Result<BuildReq, _> = serde_json::from_str(&buf);
                if req.is_err() { continue; }
                let req = req.unwrap();
                match req.kind.as_str() {
                    "control" => {
                        if let Some(cmd) = req.cmd.as_deref() {
                            match cmd {
                                "stop" => { stop.store(true, Ordering::SeqCst); let _ = s.write_all(b"{\"ok\":true}"); break; }
                                "status" => { let _ = s.write_all(b"{\"status\":\"running\"}"); }
                                _ => {}
                            }
                        }
                    }
                        let cwd = req.cwd.unwrap_or_else(|| ".".into());
                        let args = req.args.unwrap_or_else(|| vec!["build".into()]);
                        let pre = sccache_stats();
                        prewarm(&cwd);
                        let resp = run_build(&cwd, &args);
                        let post = sccache_stats();
                        let _ = s.write_all(serde_json::to_string(&BuildResp { ok: resp.ok, elapsed_ms: resp.elapsed_ms, stdout: resp.stdout, stderr: resp.stderr, sccache_pre: pre, sccache_post: post }).unwrap().as_bytes());
                    }
                    _ =>  
                }
            }
            Err(e) => { warn!("connection error: {}", e); }
        }
        std::env::set_var("RUSTC_WRAPPER", "sccache");
        std::env::set_var("CARGO_INCREMENTAL", "0");
    } else {
        std::env::set_var("CARGO_INCREMENTAL", "1");
    }
    prewarm(cwd);
    let pre = sccache_stats();
    let start = Instant::now();
    let cargo_home = std::path::Path::new(cwd).join("tools").join("noinc-cargo-home");
    let output = std::process::Command::new("cargo")
        .env("RUSTC_WRAPPER", "sccache")
        .env("CARGO_INCREMENTAL", "0")
        .args(args).current_dir(cwd)
        .stdout(Stdio::piped()).stderr(Stdio::piped()).output();
    let elapsed = start.elapsed().as_millis();
    let post = sccache_stats();
    prewarm(cwd);
    match output {
        Ok(o) => {
            let mut stdout = String::from_utf8_lossy(&o.stdout).to_string();
            let mut stderr = String::from_utf8_lossy(&o.stderr).to_string();
            if stdout.len() > 64_000 { stdout.truncate(64_000); }
            if stderr.len() > 64_000 { stderr.truncate(64_000); }
            BuildResp { ok: o.status.success(), elapsed_ms: elapsed, stdout, stderr, sccache_pre: pre, sccache_post: post }
        }
        Err(e) => BuildResp { ok: false, elapsed_ms: elapsed, stdout: String::new(), stderr: e.to_string(), sccache_pre: pre, sccache_post: post },
    }
}

fn spawn_watcher(path: String) {
    std::thread::spawn(move || {
        use notify::{RecommendedWatcher, Watcher, RecursiveMode};
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = match RecommendedWatcher::new(tx, notify::Config::default()) {
            Ok(w) => w,
            Err(_) => return,
        };
        let _ = watcher.watch(std::path::Path::new(&path), RecursiveMode::NonRecursive);
        let mut last = std::time::Instant::now() - Duration::from_secs(3600);
        loop {
            match rx.recv() {
                Ok(Ok(ev)) => {
                    let s = format!("{:?}", ev.paths);
                    if s.contains("Cargo.lock") && last.elapsed() > Duration::from_secs(5) {
                        tracing::info!("Cargo.lock changed; prewarming dependencies…");
                        prewarm(&path);
                        last = std::time::Instant::now();
                    }
                }
                _ => break,
            }
        }
    });
}
