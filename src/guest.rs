use std::net::{TcpListener, TcpStream};

pub struct Guest {
    guest_id: u32,
}

impl Guest {
    fn init(guest_id: u32) -> Guest {
        Guest {
            guest_id,
        }
    }
    
    fn getID(&self) -> u32{
        self.guest_id
    }

    fn bind(&self, port: String) -> TcpListener{
        TcpListener::bind(port.as_str()).unwrap()
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