
use clap::{Parser};
use crate::internal::enums::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli{
    #[command(subcommand)]
    pub command:Commands
}