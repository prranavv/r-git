use crate::{Result, internal::utils::init_repo};

pub fn init()->Result<()>{
    init_repo()?;
    Ok(())
}