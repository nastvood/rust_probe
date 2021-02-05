use std::env::args;
use std::process;

//use futures::executor::block_on;

use ws_srv::Config;
use ws_srv::Server;

fn main() {

    let args: Vec<String> = args().collect();

    let config = Config::by_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let mut server = Server::new(config);
    server.run().unwrap_or_else(|err| {
        eprintln!("Problem runing server: {}", err);
        process::exit(1)
    }); 
}
