use webcache::{server::Server, cache::Cache, guest::Guest};
use std::{thread, net::TcpStream, io::Write, time::Duration};

#[allow(unused)]
fn main() {
    // Create web server and web cache
    // bind server and cache to localhost:3001 and localhost:3002

    // thread::spawn(|| {
    //     let mut web_server = Server::init(0);
    //     let server_listener = web_server.bind("127.0.0.1:3001".to_string());
    //     for stream in server_listener.incoming() {
    //         let stream = stream.unwrap();

    //         web_server.handle_stream(stream);
    //     }
    // });

    thread::spawn(|| {
        let mut web_cache = Cache::init(5);
        let cache_listener = web_cache.bind("127.0.0.1:3002".to_string());
        for stream in cache_listener.incoming() {
            let stream = stream.unwrap();

            web_cache.handle_stream(stream);
        }
    });

    // Create N clients using threads

    // loop{
    //     if let Ok(stream) = TcpStream::connect("127.0.0.1:3001") {
    //         println!("Connect successfully!");
    //         break;
    //     } else
    //     {
    //         println!("Connect Error!");
    //     }
    // }

    thread::spawn(|| {
        let mut guest = Guest::init(0);
        let guest_listener = guest.bind("127.0.0.1:3003".to_string());
        let mut stream = TcpStream::connect("127.0.0.1:3002");
        match stream {
            Ok(mut stream) => {
                println!("Connect successfully!");
                let contents = "owo".to_string();
                stream.write_all(contents.as_bytes());
            },
            Err(_) => {
                println!("Connect ERROR!");
            }
        }
    });

    // main tasks

    // close all clients

    // close web server and web cache

    thread::sleep(Duration::from_millis(100));

}
