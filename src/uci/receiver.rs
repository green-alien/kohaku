
use std::io;
use std::str::Split;

mod commands; //reciver/commands.rs

/** spawn
 * spawn a thread that listens to stdio and sends incoming commands to a parser
 */
pub fn spawn() -> () {
    std::thread::Builder::new()
        .name("listener-stdio".to_string())
        .spawn(|| {
            loop {
                let mut buff = String::new();
                io::stdin()
                    .read_line(&mut buff)
                    .unwrap();
                parse_uci(buff);
            }   
        }).unwrap();
}

/** parse_uci
 * read incomming commands and execute related internal interface
 */
pub fn parse_uci(comm: String) -> () {
    std::thread::spawn(move || {
        let clean = comm.as_str().replace("\n", "");
        let command = clean.split(" ");

        fn parse(mut args: Split<&str>) -> () {
            let alpha = args.next().unwrap();
            let args = args;
            match alpha {
                "uci"        => commands::uci(),
                "isready"    => commands::isready(),
                "register"   => commands::register(),
                "ucinewgame" => commands::ucinewgame(),
                "stop"       => commands::stop(),
                "ponderhit"  => commands::ponderhit(),
                "quit"       => commands::quit(),
                "debug"      => commands::debug(args),
                "position"   => commands::position(args),
                "go"         => commands::go(args),
                "setoption"  => commands::setoption(args),
                _            => parse(args) // recursive call to drop unrecognized tokens
            }
        }

        parse(command);
    });
}
