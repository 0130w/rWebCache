use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, collections::HashMap};

pub struct Server {
    device_type: String,
    server_id: u32,
    addr: String,
    storage: HashMap<String, String>
}

impl Server {
    /**
     * ```shell
     * Create a server using server_id with default addr "0.0.0.0"
     * ```
     *  */ 
    pub fn init(server_id: u32) -> Self{
        Server { 
            device_type: "Server".to_string(),
            server_id, 
            addr: "0.0.0.0".to_string(),
            storage: HashMap::new()
        }
    }

    pub fn get_device_type(&self) -> &String {
        &self.device_type
    }

    /**
     * ```shell
     * get server itself's id
     * ```
     */
    pub fn get_id(&self) -> u32 {
        self.server_id
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    /**
     * ```shell
     * bind server to an IP address
     * ```
     */
    pub fn bind(&mut self, addr: String) -> TcpListener{
        self.addr = addr;
        TcpListener::bind(&self.addr.as_str()).unwrap()
    }

    /**
     * ```shell
     * start to TCP listen
     * use after bind function
     * ```
     */
    pub fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
        }
    }

    /**
     * ```shell
     * handle stream from web cache
     * ```
     */
    pub fn handle_stream(&self, mut stream: TcpStream) {
        //todo!
        let buf_reader = BufReader::new(&mut stream);
        let mut lines = buf_reader.lines();
        let device_type = self.get_device_type().to_string();
        let line = lines.next().unwrap().unwrap();
        let value = self.storage.get(line.as_str());    // line is map_key , value is map_value
        match value {
            Some(value) => {
                let response = device_type + "\n" + line.as_str() + "\n" + value.as_str();
                let mut stream = TcpStream::connect("127.0.0.1:3002").unwrap();
                stream.write_all(response.as_bytes()).unwrap();
            },
            None => {
                panic!("Not Found in server!");
            }
        }
    }

    /**
     * ```shell
     * write contents back to web cache
     * ```
     */
    pub fn write(&self) {
        
    }

}