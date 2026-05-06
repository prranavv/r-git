use crate::{
    Result, error::RGitError, internal::{
        commit_hash::CommitHash,
        utils::{get_commits_for_branch, get_head_branch_name},
    }
};

pub fn merge(branch_name: &String) -> Result<()> {
    let branch_commits = get_commits_for_branch(branch_name)?;
    let parent_branch = get_head_branch_name().ok_or(RGitError::NoHeadBranch)?;
    let parent_commits = get_commits_for_branch(&parent_branch)?;
    let mut branch_commits_iter = branch_commits.iter();
    let mut first_common_commit: Option<&CommitHash> = None;
    while let Some(x) = branch_commits_iter.next() {
        if parent_commits.contains(&x) {
            first_common_commit = Some(x);
            break;
        }
    }
    if let None = first_common_commit {
        return Err(RGitError::NoCommonCommit);
    }
    println!("{:?}", first_common_commit);
    Ok(())
}
