use std::path::PathBuf;

use clap::{Subcommand};

#[derive(Subcommand)]
#[command(version, about, long_about = None)]
pub enum Commands{
    Init,
    HashObject{
        name:String,
        #[arg(short,long)]
        write:bool,
        #[arg(short,long)]
        stdin:bool
    },
    CatFile{
        hash:String,
        #[arg(short='p')]
        pretty_print:bool,
        #[arg(short)]
        type_of: bool
    },
    WriteTree,
    LsTree{
        name:String
    },
    CommitTree{
        tree_hash:String,
        #[arg(short,long)]
        message:String
    },
    Log,
    Checkout{
        commit_hash:String
    },
    Branch{
        branch_name: String
    },
    Add{
        file_name: PathBuf
    },
    Commit{
        #[arg(short='m')]
        message:String,
        #[arg(short='a')]
        all:bool,
    },
    Status,
    Rm{
        file_path:PathBuf
    }
}