use webcache::{server::Server, cache::Cache};
use std::thread;

#[allow(unused)]
fn main() {
    // Create web server and web cache
    // bind server and cache to localhost:3001 and localhost:3002

    thread::spawn(|| {
        let mut web_server = Server::init(0);
        let server_listener = web_server.bind("127.0.0.1:3001".to_string());
        for stream in server_listener.incoming() {
            let stream = stream.unwrap();

            web_server.handle_stream(stream);
        }
    });

    thread::spawn(|| {
        let mut web_cache = Cache::init(0);
        let cache_listener = web_cache.bind("127.0.0.1:3002".to_string());
        for stream in cache_listener.incoming() {
            let stream = stream.unwrap();

            web_cache.handle_stream(stream);
        }
    });

    // Create N clients using threads

    

    // main tasks

    // close all clients

    // close web server and web cache
}
