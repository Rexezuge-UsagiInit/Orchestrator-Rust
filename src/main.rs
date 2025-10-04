mod ipc;

fn main() {
    match ipc::socket::create::create() {
        Ok(()) => println!("Socket created successfully"),
        Err(e) => eprintln!("Error creating socket: {}", e),
    }
}
