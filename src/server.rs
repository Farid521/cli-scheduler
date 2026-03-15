use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("client connected");
    let mut buff = [0; 1024];
    let data_length = stream.read(&mut buff)?;

    let response = 
        "HTTP/1.1 200 OK\r\n\
         Content-Length: 12\r\n\
         Content-Type: text/plain\r\n\
         \r\n\
         Hello World!";

    let request = String::from_utf8_lossy(&buff[..data_length]);
    println!("client url: {}", request.lines().next().unwrap());

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
        
    println!("data length: {:?}", data_length);

    Ok(())
}

pub fn create_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8090")?;

    println!("tcp server started");

    for stream in listener.incoming() {
        match stream{
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    println!("Error in handling connection : {}", e);
                }
            } 
            Err(e) => println!("connection failed: {}", e),
        }
    }
    Ok(())
}