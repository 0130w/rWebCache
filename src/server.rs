use std::net::{TcpListener, TcpStream};

pub struct Server {
    server_id: u32,
    addr: String
}

impl Server {
    fn init(server_id: u32) -> Self{
        Server { 
            server_id, 
            addr: "0.0.0.0".to_string() 
        }
    }

    fn get_id(&self) -> u32 {
        self.server_id
    }

    fn bind(&mut self, addr: String) -> TcpListener{
        self.addr = addr;
        TcpListener::bind(&self.addr.as_str()).unwrap()
    }

    fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
        }
    }

    fn handle_stream(&self, stream: TcpStream) {
        // todo!
    }

    fn write(&self) {
        
    }

}