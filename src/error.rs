use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum RGitError{
    #[error("error: file does not exist: {path} \ncaused by the error: {source}")]
    FileDoesNotExist{
        path:PathBuf,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to read the file :{path} \ncaused by the error: {source}")]
    FileReadError{
        path:PathBuf,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to write the file :{path} \ncaused by the error: {source}")]
    FileWriteError{
        path:PathBuf,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to create directory: {path} \ncaused by the error: {source}")]
    DirectoryCreateError{
        path:PathBuf,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to read directory: {path} \ncaused by the error: {source}")]
    DirectoryReadError{
        path:PathBuf,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to read entry within directory: {path} \ncaused by the error: {source}")]
    DirectoryEntryError{
        path:PathBuf,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to convert bytes to string \ncaused by the error: {source}")]
    BytesToStringError{
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: the file is not a tree")]
    NotTree,

    #[error("error: the file is not a commit hash")]
    NotCommitHash,

    #[error("error: options '-t' and '-p' cannot be used togetherH")]
    CantPrettyPrintAndType,
    
    #[error("error: a branch named {branch_name} already exists")]
    BranchAlreadyExists{
        branch_name:String
    },

    #[error("error: not a valid hash : {hash_name}")]
    NotValidHash{
        hash_name: String
    },

    #[error("error: not a valid index entry: {index_entry} ")]
    NotValidIndexEntry{
        index_entry: String
    },

    #[error("error: erroring parsing integer \ncaused by the error: {source}")]
    ParseIntError{
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: error getting currend date time")]
    CantGetDateTime,

    #[error("error: cannot create a new branch without having atleast 1 commit")]
    CantCreateNewBranch,

    #[error("error: cannot get head branch")]
    CantGetHeadBranch,

    #[error("io error: {0}")]
    Io(#[from] io::Error),    
}

