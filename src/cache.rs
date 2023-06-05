use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, collections::HashMap};

use crate::guest::{Guest, self};

pub struct Cache {
    device_type: String,
    cache_size: u32,
    cache: HashMap<String, String>,
    current_size: u32
}

impl Cache {
    pub fn init(cache_size: u32) -> Cache {
        Cache {
            device_type: "Cache".to_string(),
            cache_size,
            cache: HashMap::new(),
            current_size: 0
        }
    }

    pub fn get_device_type(&self) -> &String {
        &self.device_type
    }

    pub fn get_size(&self) -> u32 {
        self.current_size
    }

    pub fn judge_size(&self) -> bool {
        self.current_size <= self.cache_size
    }

    pub fn bind(&self, addr: String) -> TcpListener {
        TcpListener::bind(addr.as_str()).unwrap()
    }

    pub fn listen(&mut self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream)
        }
    }

    pub fn handle_stream(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let mut lines = buf_reader.lines();
        let device_type = lines.next().unwrap().unwrap();
        println!("device_type = {device_type}");
        let line = lines.next().unwrap().unwrap();  // line is map_key
        //println!("{line}");
        //self.cache.insert(line.clone(), "qwq".to_string());
        if device_type  == "Guest"{
                let value = self.cache.get(line.as_str().clone());

                match value {
                    Some(value) => {
                    // found in cache
                    println!("{value} found in cache!");
                    },
                    None => {
                        let server_stream = TcpStream::connect("127.0.0.1:3001");
                        match server_stream {
                            Ok(mut server_stream) => {
                                println!("Connect server successfully!");
                                server_stream.write_all(line.as_bytes()).unwrap();    // line is map_key
                            },
                            Err(_) => {
                                println!("Connect server Error!");
                            }
                        }
                    }
                }
        } else if device_type == "Server"{
                let value = lines.next().unwrap().unwrap();
                self.cache.insert(line.clone(), value.clone());
                println!("Get value from server: {}", value);
        } else {
            panic!("Not support device type!");
        }
        
        //println!("{value}");
    }

    pub fn check_status(&self, buf: BufReader<&TcpStream>) -> bool{
        let status_line = buf.lines().next().unwrap().unwrap();
        if status_line == "GET / HTTP/1.1" {
            return true;
        }
        return false;
    }
}