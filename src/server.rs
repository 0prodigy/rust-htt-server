use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr:String) -> Self {
        Self {
          addr
        }
    }
    pub fn run(&self) {
        println!("Server is running on {}",self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept(){
                Ok((mut stream,_)) =>{
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{}",String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => {
                            println!("failed to read stream Error {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to create connection with tcp Error {}",e)
                }
            }
        }
    }
}
