use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

fn main() {
    // // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // Read the request from the stream
                let buf_reader = BufReader::new(&_stream);
                let request_line = buf_reader.lines().next().unwrap().unwrap();

                let mut request_buffer_it = request_line.split_ascii_whitespace();

                let _http_verb = request_buffer_it.next().unwrap_or("GET");
                let request_target = request_buffer_it.next().unwrap_or("/");

                let response = match request_target {
                    "/" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };

                // Write the response to stream
                _stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
