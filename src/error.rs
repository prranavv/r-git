use std::io;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum RGitError{
    #[error("error: file does not exist: {path} \ncaused by the error: {source}")]
    FileDoesNotExist{
        path:String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to read the file :{path} \ncaused by the error: {source}")]
    FileReadError{
        path:String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to write the file :{path} \ncaused by the error: {source}")]
    FileWriteError{
        path:String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to create directory: {path} \ncaused by the error: {source}")]
    DirectoryCreateError{
        path:String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to read directory: {path} \ncaused by the error: {source}")]
    DirectoryReadError{
        path:String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("error: failed to read entry within directory: {path} \ncaused by the error: {source}")]
    DirectoryEntryError{
        path:String,
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
    
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    
}