use std::{
    net::{TcpListener},
    env
};

pub mod monothreaded;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server_type = &args[1];
    let service_version = &args[2];

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if server_type == "mono" {
            if service_version == "v01" {
                monothreaded::v01::handle_connection(stream);
            }
        }
        
    }
}

