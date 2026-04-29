use std::path::PathBuf;

use clap::{Subcommand};

#[derive(Subcommand)]
#[command(version="1.0.0", about, long_about = "Rust Implementation of Git")]
pub enum Commands{
    /// Create an empty Git repository or reinitialize an existing one
    Init,
    /// Compute object ID and optionally create an object from a file
    HashObject{
        name:String,
        #[arg(short,long)]
        write:bool,
        #[arg(short,long)]
        stdin:bool
    },
    /// Provide contents or details of repository objects
    CatFile{
        hash:String,
        #[arg(short='p')]
        pretty_print:bool,
        #[arg(short)]
        type_of: bool
    },
    /// Create a tree object from the current index
    WriteTree,
    /// List the contents of a tree object
    LsTree{
        name:String
    },
    /// Create a new commit object
    CommitTree{
        tree_hash:String,
        #[arg(short,long)]
        message:String
    },
    /// Show commit logs
    Log,
    /// Switch branches or restore working tree files
    Checkout{
        commit_hash:String
    },
    /// Create a new branch
    Branch{
        branch_name: String
    },
    /// Add file contents to the index
    Add{
        file_name: PathBuf
    },
    /// Record changes to the repository
    Commit{
        #[arg(short='m')]
        message:String,
        #[arg(short='a')]
        all:bool,
    },
    /// Show the working tree status
    Status,
    /// Remove files from the working tree and from the index
    Rm{
        file_path:PathBuf,
        #[arg(long="cached")]
        cached:bool,
    }
}