use std::net::{TcpListener, TcpStream};

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
}