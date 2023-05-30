use std::net::{TcpListener, TcpStream};

pub struct Server {
    server_id: u32,
    addr: String
}

impl Server {
    /**
     * ```shell
     * Create a server using server_id with default addr "0.0.0.0"
     * ```
     *  */ 
    pub fn init(server_id: u32) -> Self{
        Server { 
            server_id, 
            addr: "0.0.0.0".to_string() 
        }
    }

    /**
     * ```shell
     * get server itself's id
     * ```
     */
    pub fn get_id(&self) -> u32 {
        self.server_id
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
    pub fn handle_stream(&self, stream: TcpStream) {
        // todo!
    }

    /**
     * ```shell
     * write contents back to web cache
     * ```
     */
    pub fn write(&self) {
        
    }

}