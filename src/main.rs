extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs;

extern crate hyper;
extern crate iron;

use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    port: u16
}

fn main() {
    let contents = fs::read_to_string("config/config.json")
        .expect("Something went wrong reading the file");

    let cfg: Config = serde_json::from_str(&contents).unwrap();

    let address = format!("127.0.0.1:{}",  cfg.port); 
    println!("\n{:?} \n\n-----------", address);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}