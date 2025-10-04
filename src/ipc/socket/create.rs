use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf}; // <-- ADDED: Path and PathBuf
use std::error::Error;
use std::fmt;

const DEFAULT_SOCKET_PATH: &str = "/tmp/UsagiInit.sock";

// Define the custom error type
#[derive(Debug)]
pub enum SocketError {
    PathError(String),
    Io(io::Error),
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SocketError::PathError(msg) => write!(f, "Path Error: {}", msg),
            SocketError::Io(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl Error for SocketError {}

// Helper to convert io::Error into SocketError
impl From<io::Error> for SocketError {
    fn from(err: io::Error) -> SocketError {
        SocketError::Io(err)
    }
}


// Corrected function signature: 
// Takes Option<&str> and returns a Result using the custom SocketError.
pub fn create(abs_path: Option<&str>) -> Result<(), SocketError> {
    let socket_path = match abs_path {
        Some(path) => {
            let p = PathBuf::from(path);
            if !p.is_absolute() {
                return Err(SocketError::PathError("Path must be absolute".to_string()));
            }
            p
        }
        None => PathBuf::from(DEFAULT_SOCKET_PATH),
    };

    // Pass a reference to the PathBuf, which automatically coerces to &Path
    // Convert the io::Error from the helper function into the custom SocketError
    create_socket_file(&socket_path)?;
    Ok(())
}

// Corrected function signature: 
// Takes a &Path (a reference to the path object) and returns io::Result<()>.
fn create_socket_file(socket_path: &Path) -> Result<(), io::Error> {
    // Note: The OpenOptionsExt::mode method is specific to Unix-like operating systems.
    let file = OpenOptions::new()
        .write(true)   
        .read(true)    
        .create(true)   
        .truncate(true) 
        .mode(0o600)    
        .open(socket_path)?; // Correctly uses the &Path

    writeln!(&file, "<<Test Content>>")?;
    Ok(())
}
