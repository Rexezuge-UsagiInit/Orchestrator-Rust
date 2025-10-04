use std::env;

mod socket_manager;
use socket_manager::SocketManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create socket manager with default path
    let manager = SocketManager::new(None)?;
    println!("Socket path: {}", manager.socket_path().display());
    println!("Environment variable USAGI_SOCKET: {}", env::var("USAGI_SOCKET")?);

    // Create the socket file
    manager.create_socket()?;
    println!("Socket created successfully");

    // Clean up
    manager.remove_socket()?;
    println!("Socket removed successfully");

    // Example with custom path
    let custom_manager = SocketManager::new(Some("/tmp/custom.sock"))?;
    println!("Custom socket path: {}", custom_manager.socket_path().display());
    println!("Environment variable USAGI_SOCKET: {}", env::var("USAGI_SOCKET")?);

    Ok(())
}
