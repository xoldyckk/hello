# Hello

## About

A multithreaded http web server example written in rust. Taught as a mini project in [chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) of the [official rust book](https://doc.rust-lang.org/book/).

Basically, the point was to apply both the beginner and advanced concepts learned throughout the book in one go. Examples of the advanced concepts are:- smart pointers, atomic smart pointers(thread-safe implementation of smart pointers), fearless concurrency provided by rust etc.

## Steps to set up and run

1. [Set up rust on your system](https://www.rust-lang.org/tools/install)

2. Clone this repository:-

   ```
   git clone https://github.com/xoldyckk/hello.git
   ```

3. Change directory of your terminal into the cloned project folder.

   ```
   cd ./hello
   ```

4. Run the following command to start the http web server:-

   ```
   cargo run
   ```

   The server is started at the following address:- [http://127.0.0.1:7878](http://127.0.0.1:7878)

## Considerations

1. Uses hardcoded value of `4` threads as the default number of threads available for serving concurrent requests. The number of threads can be changed when running locally. Here's how to do it:-

   Go to line `15` of the file [main.rs](./src/main.rs) and change the number
   `4` to the desired value.

   ```rust
   let pool = ThreadPool::new(4);
   ```

2. Uses hardcoded value of `20` tcp streams as the amount of streams(http requests) to respond to before shutting down the server, this is done to illustrate the concept of graceful server shut down. The number of requests to handle can be changed when running locally. Here's how to do it:-

   Go to line `21` of the file [main.rs](./src/main.rs) and change the number `20` to the desired value.

   ```rust
   for stream in listener.incoming().take(20) {
   ```

3. Uses hardcoded value of `10` seconds as the amount of time to delay the execution of thread handling the request to [http://127.0.0.1:7878/sleep](http://127.0.0.1:7878/sleep) route before responding to the client. This is done to illustrate the concept of concurrency and multi-threading provided by rust for the http web server. The amount of time to delay the request to [http://127.0.0.1:7878/sleep](http://127.0.0.1:7878/sleep) route can be changed when running locally. Here's how to do it:-

   Go to line `50` of the file [main.rs](./src/main.rs) and change the number `10` to the desired value.

   ```rust
   thread::sleep(Duration::from_secs(10));
   ```

## Routes

### http://127.0.0.1:7878

root route `/`, returns the [hello.html](./hello.html) page stored in root directory of this project.

### http://127.0.0.1:7878/sleep

`/sleep` route, also returns the [hello.html](./hello.html) page stored in root directory of this project. The difference is that this route takes at least 10 seconds to return the page.

### http://127.0.0.1:7878/**/*

`/**/*` route, denotes any route which is not the `/` or `/sleep` route, returns the [404.html](./404.html) page stored in root directory of this project.

## Testing concurrency and multi-threaded nature of the web server

Open the routes [http://127.0.0.1:7878](http://127.0.0.1:7878) and [http://127.0.0.1:7878/sleep](http://127.0.0.1:7878/sleep) in seperate browser tabs.

Make a request to [http://127.0.0.1:7878/sleep](http://127.0.0.1:7878/sleep) route in one tab and within the span of `10` seconds make a request to [http://127.0.0.1:7878](http://127.0.0.1:7878) route in another tab.

Notice that the request in second tab is fullfilled immediately while the request in first tab in first tab is still processing.

This is because the request in first tab is being handled by a different thread than the request in second tab.

So, the thread handling request for the second tab responds to it immediately and we see the [hello.html](./hello.html) page instantaneously while the thread handling request for the first tab responds it after `10` seconds.

## Additional

Throughout the codebase, there are detailed comments about why things are implemented the way they are. Also, this implementation is a teeny-tiny bit different from the implementation in the [official rust book](https://doc.rust-lang.org/book/).
