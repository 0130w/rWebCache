use std::net::{TcpListener, TcpStream};

pub struct Guest {
    guest_id: u32,
    port: String,
}

impl Guest {
    pub fn init(guest_id: u32) -> Guest {
        Guest {
            guest_id,
            port: "0.0.0.0".to_string()
        }
    }
    
    fn get_id(&self) -> u32{
        self.guest_id
    }

    fn bind(&mut self, port: String) -> TcpListener{
        self.port = port;
        TcpListener::bind(&self.port.as_str()).unwrap()
    }

    fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
        }
    }

    fn handle_stream(&self, stream: TcpStream) {
        //todo!
        
    }

    fn write(&self, listener: TcpListener) {

    }
}