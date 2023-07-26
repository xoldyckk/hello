use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // creates a tcp listener that listens for incoming tcp streams
    // at the provided address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // creates a thread pool with 4 threads
    let pool = ThreadPool::new(4);

    // listener.incoming() returns an iterator over the sequence of
    // incoming tcp streams, by default listens for incoming tcp streams indefinitely,
    // .take(20) makes it so that it only handles 20 incoming tcp streams and shuts down
    // right after, done to illustrate the concept of graceful shutdown
    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // this message can show up in random order in the console output
    // since other threads can print their own messages simultaneously
    println!("Shutting down.");
}

// this function handles an incoming tcp stream, in this project it is passed to
// a thread inside a closure each time there's a new request made to the server
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // get first line of http request, is generally of the format:-
    //
    // <http_method> <route_segment> <http_version>
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // matches a set of pre-defined routes
    let (response_status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // makes the current thread it exists in sleep for 10 seconds, intentionally
            // done here to explain the concept of multithreading i.e., to delegate
            // incoming requests to other threads if one thread is stuck on a computation
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let response_body = fs::read_to_string(file_name).unwrap();
    // \r\n is CRLF character(carriage return line feed), it seperates different
    // lines within a http request and response object, while parsing a http
    // request object an empty line with zero characters and just \r\n signifies
    // the start of request/response body(which is optional to be provided),
    // here \r\n\r\n means end the current line and next line is an empty line
    // this is the format:-
    //
    // <http_version> <status_code> <status_code_keyword>
    // Content-Length: <content_length>
    //
    // <response_body>
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        response_status_line,
        response_body.len(),
        response_body
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
