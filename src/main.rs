use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    // // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // Read the request from the stream
                let mut buf_reader = BufReader::new(&_stream).lines();
                let request_line = buf_reader.next().unwrap().unwrap();

                let mut request_buffer_it = request_line.split_ascii_whitespace();

                let _http_verb = request_buffer_it.next().unwrap_or("GET");
                let request_target = request_buffer_it.next().unwrap_or("/");

                let response = match request_target {
                    "/" => String::from("HTTP/1.1 200 OK\r\n\r\n"),
                    "/user-agent" => {
                        let user_agent = buf_reader
                            .find(|x| x.as_ref().unwrap().starts_with("User-Agent"))
                            .unwrap()
                            .unwrap();

                        let response = user_agent.trim_start_matches("User-Agent: ");
                        let response_length = response.len();
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {response_length}\r\n\r\n{response}")
                    }
                    _ if request_target.starts_with("/echo/") => {
                        let echo_response = request_target.trim_start_matches("/echo/");
                        let echo_response_length = echo_response.len();
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_response_length, echo_response)
                    }
                    _ => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
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
