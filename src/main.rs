use wuzi::board::board_server::BoardServer;
use wuzi::config::defaults::DEFAULT_ADDRESS;
use std::env;

fn main() {
    let mut addr = DEFAULT_ADDRESS;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        addr = args[1].as_str();
    }

    let mut board_server = BoardServer::new();
    board_server.serve(addr.to_string());
}
