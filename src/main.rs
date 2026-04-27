use clap::{Parser};
use crate::error::RGitError;
use std::io;
use std::io::Write;

pub mod cli;
pub mod internal;
pub mod error;

pub use cli::Cli;
pub use internal::Commands;

type Result<T> = std::result::Result<T, RGitError>;

fn main()->Result<()>{
    let cli = Cli::parse();
    let mut stderr = io::stderr();

    match &cli.command{
        Commands::Init=>{
            match internal::init(){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        },
        Commands::HashObject { name,write ,stdin}=>{
            match internal::hash_object(name, write, stdin){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        }
        Commands::CatFile { pretty_print ,type_of,hash}=>{
            match internal::cat_file(&hash,*type_of,*pretty_print){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        },
        Commands::WriteTree=>{
            match internal::write_tree(){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        },
        Commands::LsTree { name }=>{
            match internal::ls_tree(name){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        },
        Commands::CommitTree { tree_hash, message }=>{
            match internal::commit_tree(tree_hash,message){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        },
        Commands::Log=>{
            match internal::log(){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?
            }
            Ok(())
        },
        Commands::Checkout { commit_hash }=>{
            match internal::checkout(commit_hash){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?    
            }
            Ok(())
        },
        Commands::Branch { branch_name }=>{
            match internal::branch(branch_name.to_string()){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?    
            }
            Ok(())
        },
        Commands::Add { file_name }=>{
            match internal::add(file_name){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string())?  
            }
            Ok(())
        }
    }
}
