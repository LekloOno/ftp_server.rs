use std::{env, path::PathBuf};

use crate::server::FtpServer;

impl FtpServer {
    pub fn current_dir(&self) -> std::io::Result<PathBuf> {
        env::current_dir()
    }

    pub fn set_current_dir(&self, pathname: &str) -> crate::Result<()> {
        if self.protect_root {
            let current = env::current_dir()?.canonicalize()?;
            let target = PathBuf::from(pathname).canonicalize()?;
            
            if !target.starts_with(current) {
                return Err(crate::Error::PermissionDenied);
            }
        }
        
        env::set_current_dir(pathname)
            .map_err(crate::Error::from)
    }
}