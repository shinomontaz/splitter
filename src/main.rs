extern crate config;
use std::net::TcpListener;

use std::path::Path;
use config::*;

fn main() {
    let mut settings = Config::default();
    settings.merge(File::from(Path::new("config/config.json"))).unwrap();

    let address = format!("127.0.0.1:{}", settings.port); 
    println!("\n{:?} \n\n-----------", address);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}