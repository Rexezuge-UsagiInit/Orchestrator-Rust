use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;

const DEFAULT_SOCKET_PATH: &str = "/run/UsagiInit.sock";

pub fn create() -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)   
        .read(true)    
        .create(true)   
        .truncate(true) 
        .mode(0o600)    
        .open("/tmp/example.txt")?;

    writeln!(&file, "<<Test Content>>")?;
    Ok(())
}
