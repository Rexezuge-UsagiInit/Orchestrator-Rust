mod ipc;
mod socket_manager;
mod env;

fn main() {
    match ipc::socket::create::create() {
        Ok(()) => println!("Socket created successfully"),
        Err(e) => eprintln!("Error creating socket: {}", e),
    }
    env::set("USAGI_INIT","hello")
}
