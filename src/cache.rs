use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

pub struct Cache {
    cache_size: u32,
}

impl Cache {
    fn init(cache_size: u32) -> Cache {
        Cache {
            cache_size
        }
    }

    fn bind(&self, port: String) -> TcpListener {
        TcpListener::bind(port.as_str()).unwrap()
    }

    fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream)
        }
    }

    fn handle_stream(&self, stream: TcpStream) {
        
    }

    fn check_status(&self, buf: BufReader<&TcpStream>) -> bool{
        let status_line = buf.lines().next().unwrap().unwrap();
        if status_line == "GET / HTTP/1.1" {
            return true;
        }
        return false;
    }
}