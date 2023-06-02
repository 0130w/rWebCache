use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}, fmt::format};

pub struct Cache {
    cache_size: u32,
}

impl Cache {
    pub fn init(cache_size: u32) -> Cache {
        Cache {
            cache_size
        }
    }

    pub fn bind(&self, addr: String) -> TcpListener {
        TcpListener::bind(addr.as_str()).unwrap()
    }

    pub fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream)
        }
    }

    pub fn handle_stream(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let line = buf_reader.lines().next().unwrap().unwrap();
        let response = format!("{line}");
        println!("{response}");
    }

    pub fn check_status(&self, buf: BufReader<&TcpStream>) -> bool{
        let status_line = buf.lines().next().unwrap().unwrap();
        if status_line == "GET / HTTP/1.1" {
            return true;
        }
        return false;
    }
}