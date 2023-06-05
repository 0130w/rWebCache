use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

pub struct Guest {
    device_type: String,
    guest_id: u32,
    addr: String,
}

impl Guest {
    pub fn init(guest_id: u32) -> Guest {
        Guest {
            device_type: "Guest".to_string(),
            guest_id,
            addr: "0.0.0.0".to_string()
        }
    }

    pub fn get_device_type(&self) -> &str {
        self.device_type.as_str()
    }
    
    pub fn get_id(&self) -> u32{
        self.guest_id
    }

    pub fn bind(&mut self, addr: String) -> TcpListener{
        self.addr = addr;
        TcpListener::bind(&self.addr.as_str()).unwrap()
    }

    pub fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
        }
    }

    pub fn handle_stream(&self, mut stream: TcpStream) {
        //test
        let buf_reader = BufReader::new(&mut stream);
        let line = buf_reader.lines().next().unwrap().unwrap();
        println!("{line}");
    }

    #[allow(unused)]
    pub fn write(&self, listener: TcpListener) {

    }
}