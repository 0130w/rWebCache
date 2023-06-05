use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

pub struct Guest {
    device_type: String,
    guest_id: u32,
    addr: String,
}

impl Guest {
    /// Create a guest using guest_id with default addr "0.0.0.0"
    pub fn init(guest_id: u32) -> Guest {
        Guest {
            device_type: "Guest".to_string(),
            guest_id,
            addr: "0.0.0.0".to_string()
        }
    }

    /// Get the type of guest
    pub fn get_device_type(&self) -> &str {
        self.device_type.as_str()
    }

    /// return server itself's id
    pub fn get_id(&self) -> u32{
        self.guest_id
    }

    /// bind guest to an IP address
    pub fn bind(&mut self, addr: String) -> TcpListener{
        self.addr = addr;
        TcpListener::bind(&self.addr.as_str()).unwrap()
    }

    /**
     * start to TCP listen
     * use after bind function
     * currently not used
     */
    pub fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
        }
    }

    ///handle stream from web cache
    pub fn handle_stream(&self, mut stream: TcpStream) {
        //test
        let buf_reader = BufReader::new(&mut stream);
        let line = buf_reader.lines().next().unwrap().unwrap();
        println!("{line}");
    }

}