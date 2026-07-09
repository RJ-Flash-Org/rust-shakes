use clap::{Parser, Subcommand};
use std::time::Instant;
use std::process::Stdio;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Parser)]
#[command(name="cargo-hyper", bin_name="cargo hyper", version, about="Hyper build accelerator")]
struct Cli {
    /// Daemon address (env: HYPERC_ADDR)
    #[arg(global=true, long, env="HYPERC_ADDR", default_value="127.0.0.1:7878")]
    addr: String,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Build {
        #[arg(long, default_value_t=false)] release: bool,
        #[arg(long, default_value_t=false)] all_features: bool,
        #[arg(long, default_value_t=false)] no_default_features: bool,
        #[arg(long, default_value_t=false)] benches: bool,
    },
    Daemon { #[command(subcommand)] sub: DaemonCmd },
    Clean,
}

#[derive(Subcommand)]
enum DaemonCmd { Start, Stop, Status }

fn main() -> anyhow::Result<()> {
    // Cargo may invoke the subcommand with argv[1] == "hyper" (i.e., "cargo hyper ...").
    // Adjust argv accordingly so Clap sees only our real subcommands.
    let mut argv: Vec<String> = std::env::args().collect();
    if argv.get(1).map(|s| s == "hyper").unwrap_or(false) {
        argv.remove(1);
    }
    let cli = Cli::parse_from(argv);
    match cli.cmd {
        Command::Build { release, all_features, no_default_features, benches } => {
            match build_via_daemon(&cli.addr, release, all_features, no_default_features, benches) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    eprintln!("hyperc: daemon unavailable ({e}); falling back to local build");
                    return local_build(release, all_features, no_default_features, benches);
                }
            }
        }
        Command::Daemon { sub } => match sub {
            DaemonCmd::Start => {
                #[cfg(target_os="windows")]
                { let _ = std::process::Command::new("cmd").args(["/C","start","/B","hyperc-daemon","serve"]).spawn(); }
                #[cfg(not(target_os="windows"))]
                { let _ = std::process::Command::new("nohup").args(["hyperc-daemon","serve"]).stdout(Stdio::null()).stderr(Stdio::null()).spawn(); }
                println!("hyperc: daemon start requested");
            }
            DaemonCmd::Stop => {
                if let Ok(mut s) = TcpStream::connect(&cli.addr) {
                    let req = serde_json::json!({ "kind":"control","cmd":"stop" }).to_string();
                    s.write_all(req.as_bytes())?;
                    println!("hyperc: stop signal sent");
                } else {
                    println!("hyperc: daemon not running");
                }
            }
            DaemonCmd::Status => {
                match TcpStream::connect(&cli.addr) {
                    Ok(mut s) => {
                        let req = serde_json::json!({ "kind":"control","cmd":"status" }).to_string();
                        s.write_all(req.as_bytes())?;
                        s.shutdown(std::net::Shutdown::Write).ok();
                        let mut buf = String::new();
                        s.read_to_string(&mut buf).ok();
                        println!("daemon: {}", buf);
                    }
                    Err(_) => println!("daemon: not running"),
                }
            }
        },
        Command::Clean => {
            if which::which("sccache").is_ok() {
                let _ = std::process::Command::new("sccache").arg("--zero-stats").status();
            }
            let _ = std::fs::remove_dir_all("target");
            println!("cleaned target/ (and zeroed sccache stats if present)");
        }
    }
    Ok(())
}

fn build_via_daemon(addr: &str, release: bool, all_features: bool, no_default_features: bool, benches: bool) -> anyhow::Result<()> {
    let mut s = TcpStream::connect(addr)?;
    let args = build_args_vec(release, all_features, no_default_features, benches);
    let req = serde_json::json!({
        "kind":"build",
        "cwd": std::env::current_dir()?.to_string_lossy(),
        "args": args
    }).to_string();
    s.write_all(req.as_bytes())?;
    s.shutdown(std::net::Shutdown::Write).ok();
    let mut buf = String::new();
    s.read_to_string(&mut buf)?;
    let v: serde_json::Value = serde_json::from_str(&buf)?;

    let elapsed = v.get("elapsed_ms").and_then(|x| x.as_u64()).unwrap_or(0);
    println!("hyperc.build.elapsed_ms={}", elapsed);

    if let (Some(pre), Some(post)) = (v.get("sccache_pre"), v.get("sccache_post")) {
        println!("hyperc.sccache.pre={}", pre.as_str().unwrap_or(""));
        println!("hyperc.sccache.post={}", post.as_str().unwrap_or(""));
    }

    if v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false) {
        Ok(())
    } else {
        eprintln!("--- stderr ---");
        if let Some(st) = v.get("stderr").and_then(|x| x.as_str()) { eprintln!("{}", st); }
        anyhow::bail!("daemon build failed")
    }
}

fn local_build(release: bool, all_features: bool, no_default_features: bool, benches: bool) -> anyhow::Result<()> {
    let args = build_args_vec(release, all_features, no_default_features, benches);
    let has_sccache = which::which("sccache").is_ok();
    if has_sccache {
        std::env::set_var("RUSTC_WRAPPER", "sccache");
        // sccache does not support incremental compilation; disable it when using sccache
        std::env::set_var("CARGO_INCREMENTAL", "0");
    } else {
        std::env::set_var("CARGO_INCREMENTAL", "1");
    }
    let start = Instant::now();
    let cargo_home = std::env::current_dir()?.join("tools").join("noinc-cargo-home");
    let status = std::process::Command::new("cargo")
        .env("CARGO_HOME", &cargo_home)
        .env("RUSTC_WRAPPER", "sccache")
        .env("CARGO_INCREMENTAL", "0")
        .args(args)
        .stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()?;
    let elapsed = start.elapsed().as_millis();
    println!("hyperc.build.elapsed_ms={}", elapsed);
    if !status.success() { anyhow::bail!("cargo build failed"); }
    Ok(())
}

fn build_args_vec(release: bool, all_features: bool, no_default_features: bool, benches: bool) -> Vec<String> {
    let mut args = vec!["build".to_string()];
    if release { args.push("--release".into()); }
    if all_features { args.push("--all-features".into()); }
    if no_default_features { args.push("--no-default-features".into()); }
    if benches { args.push("--benches".into()); }
    args
}
