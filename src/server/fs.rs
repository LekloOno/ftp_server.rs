//! Provides methods to handle interractions with the server's file system
//! while complying to its policies.

use std::{env, path::PathBuf};
use crate::server::FtpServer;

impl FtpServer {
    pub fn current_dir(&self) -> std::io::Result<PathBuf> {
        env::current_dir()
    }

    /// Checks if the target path is valid then proceeds.
    /// 
    /// Such validation can typically be checking if the target is not a parent of root
    /// if root traversal protection is enabled.
    pub fn set_current_dir(&self, pathname: &str) -> crate::Result<()> {
        if self.protect_root {
            let target = PathBuf::from(pathname).canonicalize()?;
            if !target.starts_with(&self.root_path.canonicalize()?) {
                return Err(crate::Error::PermissionDenied);
            }
        }
        
        env::set_current_dir(pathname)
            .map_err(crate::Error::from)
    }
}