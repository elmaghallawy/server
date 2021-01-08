# server

## is a simple multithreaded TCP/HTTP server built with Rust

### server

the server listens to tcp incoming tcp streams and handle the incoming stream through a closure that contains a function to handle the incoming request and provide a proper response,
that closure is passed to a thread from the thread pool to excute.

### threadpool

the threadpool a number working threads `workers` that listens to the incoming channel of closures to excute it when it recieves any,
`threadpool` is what enable multithreading work to be handled safely so the server is enable to process concurrent requests.
