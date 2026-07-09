use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="hyperc-client", version, about="HyperC component")]
struct Cli {
    #[arg(short, long, global=true, default_value="info")]
    log: String,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about="Print status (stub)")]
    Status,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .with_target(false)
        .compact()
        .init();
    match Cli::parse().cmd {
        Command::Status => println!("{}: OK", "hyperc-client"),
    }
    Ok(())
}
