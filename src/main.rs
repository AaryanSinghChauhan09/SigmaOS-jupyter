#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-jupyter", about = "SigmaOS Jupyter session manager")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List }
fn main() { let _cli = Cli::parse(); println!("SigmaOS Jupyter: no kernels running."); }
