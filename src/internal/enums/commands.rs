use std::path::PathBuf;

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Commands{
    /// Create an empty Git repository or reinitialize an existing one
    Init,
    /// Compute object ID and optionally create an object from a file
    HashObject{
        name:String,
        /// write the object into the object database
        #[arg(short,long)]
        write:bool,
        /// read the object from stdin
        #[arg(short,long)]
        stdin:bool
    },
    /// Provide contents or details of repository objects
    CatFile{
        /// An existing object.
        hash:String,
        /// Pretty-print the contents of <object> based on its type.
        #[arg(short='p')]
        pretty_print:bool,
        /// Instead of the content, show the object type identified by <object>.
        #[arg(short)]
        type_of: bool
    },
    /// Create a tree object from the current index
    WriteTree,
    /// List the contents of a tree object
    LsTree{
        //. Id of a tree-ish.
        name:String
    },
    /// Create a new commit object
    CommitTree{
        /// An existing tree object.
        tree_hash:String,
        /// commit message
        #[arg(short,long)]
        message:String
    },
    /// Show commit logs
    Log,
    /// Switch branches or restore working tree files
    Checkout{
        /// Branch to checkout from or commit hash to checkout from.
        commit_hash:String
    },
    /// Create a new branch
    Branch{
        /// The name of the branch to create
        branch_name: String
    },
    /// Add file contents to the index
    Add{
        /// File to add to to the staging area
        file_name: PathBuf
    },
    /// Record changes to the repository
    Commit{
        /// Use <msg> as the commit message. 
        #[arg(short='m')]
        message:String,
        /// commit all changed files
        #[arg(short='a')]
        all:bool,
    },
    /// Show the working tree status
    Status,
    /// Remove files from the working tree and from the index
    Rm{
        /// Files to remove. A leading directory name (e.g.  dir to remove dir/file1 and dir/file2)
        file_path:PathBuf,
        /// Use this option to unstage and remove paths only from the index. Working tree files, whether modified or not, will be left alone.
        #[arg(long="cached")]
        cached:bool,
    },
    /// Show changes between files
    Diff{
        /// file to show the diff
        file_path:PathBuf
    }
}