use std::{
  fs,
  io::{prelude::*, BufReader},
  net::{TcpStream},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  
  let request_line = buf_reader.lines().next().unwrap().unwrap();

  let (status_line, filename) = match &request_line[..] {
      "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello-world.html"),
      "GET /hello-world.html HTTP/1.1" => ("HTTP/1.1 200 OK", "hello-world.html"),
      "GET /hello-universe.html HTTP/1.1" => ("HTTP/1.1 200 OK", "hello-universe.html"),
      _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
  };

  let contents = fs::read_to_string(format!("../http_root/{}",filename)).unwrap();
  let length = contents.len();

  let response =
      format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\nServer: Opendojo-rust-web-mono-v01/{VERSION}\r\n\r\n{contents}");

  stream.write_all(response.as_bytes()).unwrap();
}