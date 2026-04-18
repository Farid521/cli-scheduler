use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

#[derive(Debug)]
pub enum ServerError {
    Io(std::io::Error),
    Auth(crate::auth::AuthError),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Io(e) => write!(f, "IO error: {}", e),
            ServerError::Auth(e) => write!(f, "Auth error: {}", e),
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServerError::Io(e) => Some(e),
            ServerError::Auth(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::Io(err)
    }
}

impl From<crate::auth::AuthError> for ServerError {
    fn from(err: crate::auth::AuthError) -> Self {
        ServerError::Auth(err)
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ServerError> {
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
    let request_method_and_querry = request.lines().next().unwrap_or("");
    let querry = request_method_and_querry.split_whitespace().nth(1).unwrap_or("");

    println!("client querry: {}", querry);

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

fn start_server() -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:8090")?;

    let (mut stream, _) = listener.accept()?;
    let mut stream_buff = [0; 4096];
    let n_buff = stream.read(&mut stream_buff)?;
    let request = String::from_utf8_lossy(&stream_buff[..n_buff]);

    let code = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|path| {
            path.split('?').nth(1)
                .and_then(|query| {
                    query.split('&').find(|p| p.starts_with("code="))
                          .map(|p| p["code=".len()..].to_string())
                })
        });

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                    <h2>Autentikasi sukses, kamu bisa tutup tab ini.</h2>";

    stream.write_all(response.as_bytes())?;  
    stream.flush()?;

    println!("code: {:?}", code);

    crate::auth::get_token(&code.unwrap_or("null".to_string()))?;

    Ok(())
}

pub fn create_server() -> Result<(), ServerError> {
    start_server()?;
    println!("tcp server started in background");
    Ok(())
}