
use clap::{Parser};
use crate::internal::enums::Commands;

#[derive(Parser)]
#[command(name="rgit")]
#[command(version="1.0.0")]
#[command(about = "Rust Implementation of Git", long_about = None)]
pub struct Cli{
    #[command(subcommand)]
    pub command:Commands
}