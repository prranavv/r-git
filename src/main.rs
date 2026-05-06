use clap::{Parser};
use crossterm::style::Stylize;
use crate::error::RGitError;
use crate::internal::commit_hash::CommitHash;
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
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::HashObject { name,write ,stdin}=>{
            match internal::hash_object(name, write, stdin){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        }
        Commands::CatFile { pretty_print ,type_of,hash}=>{
            match internal::cat_file(&hash,*type_of,*pretty_print){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::WriteTree=>{
            match internal::write_tree(){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::LsTree { name }=>{
            match internal::ls_tree(name){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::CommitTree { tree_hash, message }=>{
            match internal::commit_tree(tree_hash,message){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Log=>{
            match internal::log(){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Checkout { commit_hash }=>{
            match internal::checkout(&CommitHash::new(commit_hash.to_string())){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Branch { branch_name ,a}=>{
            match internal::branch(branch_name.clone(),*a){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Add { file_name }=>{
            match internal::add(file_name){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Commit { message ,all}=>{
            match internal::commit(message,*all){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Status=>{
            match internal::status(){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Rm { file_path,cached }=>{
            match internal::rm(file_path,*cached){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Diff { file_path }=>{
            match internal::diff(file_path){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        },
        Commands::Merge { branch_name }=>{
            match internal::merge(branch_name){
                Ok(_)=>{},
                Err(e)=>writeln!(stderr,"{}",e.to_string().red())?
            }
            Ok(())
        }
    }
}
