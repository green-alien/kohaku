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

    let init_pos = chess::initial_position_state();

    println!("{:?}", init_pos);

    println!("{}", init_pos.board.white_king);
    
    let ten_seconds = std::time::Duration::from_secs(1);


    for i in chess::rook_lookup().iter() {
        for (e, a) in i.iter().enumerate() {
            //println!("{} {}", e, *a);
            thread::sleep(ten_seconds);

        } 
    }

    let ten_seconds = std::time::Duration::from_secs(10);
    thread::sleep(ten_seconds);
    
    println!("Program exit.");
}
