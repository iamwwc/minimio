use std::{io::Read, net::{TcpListener, TcpStream}};
use std::io::Result;
fn handle_connection (mut stream: TcpStream) {
    let mut buffer = [0;1024];
    match stream.read(&mut buffer) {
        Err(e) => {
            println!("failed");
        }
        Ok(v) => {
            
            println!("{:?}",std::str::from_utf8(&buffer[0..v]).unwrap());
        }
    }
}

fn main() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:80")?;
    for stream in listener.incoming() {
        handle_connection(stream?);
    }
    Ok(())
}