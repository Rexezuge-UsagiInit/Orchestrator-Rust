mod ipc;
mod env;

fn main() {
    match ipc::socket::create(None) {
        Ok(_) => println!("Socket created successfully"),
        Err(e) => eprintln!("Error creating socket: {}", e),
    }
    env::set("USAGI_INIT","hello")
}
