use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const DEFAULT_SOCKET_PATH: &str = "/run/UsagiInit.sock";
const ENV_VAR_NAME: &str = "USAGI_SOCKET";

#[derive(Debug)]
pub enum SocketError {
    IoError(std::io::Error),
    PathError(String),
}

impl From<std::io::Error> for SocketError {
    fn from(err: std::io::Error) -> Self {
        SocketError::IoError(err)
    }
}

pub struct SocketManager {
    socket_path: PathBuf,
}

impl SocketManager {
    pub fn new(custom_path: Option<&str>) -> Result<Self, SocketError> {
        let socket_path = match custom_path {
            Some(path) => {
                let p = PathBuf::from(path);
                if !p.is_absolute() {
                    return Err(SocketError::PathError("Path must be absolute".to_string()));
                }
                p
            }
            None => PathBuf::from(DEFAULT_SOCKET_PATH),
        };

        unsafe {
            env::set_var(ENV_VAR_NAME, &socket_path);
        }
        Ok(SocketManager { socket_path })
    }

    pub fn create_socket(&self) -> Result<(), SocketError> {
        if let Some(parent) = self.socket_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::File::create(&self.socket_path)?;
        let metadata = fs::metadata(&self.socket_path)?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&self.socket_path, permissions)?;

        Ok(())
    }

    pub fn remove_socket(&self) -> Result<(), SocketError> {
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path)?;
        }
        Ok(())
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_new_with_default_path() {
        let manager = SocketManager::new(None).unwrap();
        assert_eq!(manager.socket_path(), Path::new(DEFAULT_SOCKET_PATH));
        assert_eq!(env::var(ENV_VAR_NAME).unwrap(), DEFAULT_SOCKET_PATH);
    }

    #[test]
    fn test_new_with_custom_path() {
        let custom_path = "/tmp/custom.sock";
        let manager = SocketManager::new(Some(custom_path)).unwrap();
        assert_eq!(manager.socket_path(), Path::new(custom_path));
        assert_eq!(env::var(ENV_VAR_NAME).unwrap(), custom_path);
    }

    #[test]
    fn test_new_with_relative_path_fails() {
        let result = SocketManager::new(Some("relative/path.sock"));
        assert!(matches!(result, Err(SocketError::PathError(_))));
    }

    #[test]
    fn test_create_and_remove_socket() {
        let temp_dir = tempdir().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let manager = SocketManager::new(Some(socket_path.to_str().unwrap())).unwrap();

        manager.create_socket().unwrap();
        assert!(socket_path.exists());

        let metadata = fs::metadata(&socket_path).unwrap();
        assert_eq!(metadata.permissions().mode() & 0o777, 0o600);

        manager.remove_socket().unwrap();
        assert!(!socket_path.exists());
    }

    #[test]
    fn test_remove_nonexistent_socket() {
        let temp_dir = tempdir().unwrap();
        let socket_path = temp_dir.path().join("nonexistent.sock");
        let manager = SocketManager::new(Some(socket_path.to_str().unwrap())).unwrap();

        let result = manager.remove_socket();
        assert!(result.is_ok());
    }
}
