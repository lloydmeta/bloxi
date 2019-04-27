extern crate bloxi_core;

use bloxi_core::Server;
use std::error::Error;
use std::{env, process};
extern crate pretty_env_logger;

fn main() -> Result<(), Box<Error>> {
    pretty_env_logger::init();
    let port_str = env::var("PORT").unwrap_or(String::from("8088"));
    let port = port_str.parse()?;
    let server = Server::new(port);
    let exit_code = server.run()?;
    if exit_code == 0 {
        Ok(())
    } else {
        process::exit(exit_code)
    }
}
