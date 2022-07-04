use std::thread;
mod uci;
mod config;
mod chess;


fn runtime_init() -> () {
    uci::receiver::spawn();
    config::set_debug(false);
    chess::init_attack_tables();
}

fn main() {
    runtime_init();

    let ten_seconds = std::time::Duration::from_secs(10);
    thread::sleep(ten_seconds);
    
    println!("Program exit.");
}
