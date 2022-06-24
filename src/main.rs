use std::thread;
mod uci;
mod config;


fn runtime_init() -> () {
    uci::receiver::spawn();
}

fn main() {
    runtime_init();

    let ten_seconds = std::time::Duration::from_secs(10);
    thread::sleep(ten_seconds);
    
    println!("Program exit.");
}
