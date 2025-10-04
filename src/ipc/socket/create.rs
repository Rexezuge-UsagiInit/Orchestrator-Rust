use std::os::unix::fs::OpenOptionsExt;

pub fn create() -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)   
        .read(true)    
        .create(true)   
        .truncate(true) 
        .mode(0o600)    
        .open("example.txt")?;

    writeln!(&file, "<<Test Content>>")?;
    Ok(())
}
