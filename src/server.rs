use std::net::TcpListener;
use std::io::{Read, Write};
use std::time::Duration;
use crate::errors::{ServerError, AuthError};

const TIMEOUT_SECS: u64 = 120;

fn start_server() -> Result<String, ServerError> {
    let listener = TcpListener::bind("127.0.0.1:8090")?;
    listener.set_nonblocking(false)?;

    // Timeout: berhenti setelah TIMEOUT_SECS jika tidak ada koneksi
    println!("Menunggu konfirmasi dari browser... (timeout: {}s)", TIMEOUT_SECS);

    // Gunakan accept dengan timeout via thread
    let (mut stream, _) = {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let listener_clone = listener.try_clone()?;

        thread::spawn(move || {
            let result = listener_clone.accept();
            let _ = tx.send(result);
        });

        match rx.recv_timeout(Duration::from_secs(TIMEOUT_SECS)) {
            Ok(Ok(conn)) => conn,
            Ok(Err(e)) => return Err(ServerError::Io(e)),
            Err(_) => return Err(ServerError::Timeout),
        }
    };

    let mut stream_buff = [0; 4096];
    let n_buff = stream.read(&mut stream_buff)?;
    let request = String::from_utf8_lossy(&stream_buff[..n_buff]);

    // Parsing path dan query dari request HTTP (contoh: GET /?code=... HTTP/1.1)
    let first_line = request.lines().next().unwrap_or("");
    let path_with_query = first_line.split_whitespace().nth(1).unwrap_or("/");

    let code = path_with_query
        .split('?')
        .nth(1)
        .and_then(|query| {
            query.split('&')
                .find(|p| p.starts_with("code="))
                .map(|p| p["code=".len()..].to_string())
        });

    let (response, result_code) = match code {
        Some(c) => (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
             <h2>Autentikasi Berhasil!</h2><p>Kamu bisa menutup tab ini dan kembali ke terminal.</p>",
            Ok(c)
        ),
        None => (
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
             <h2>Autentikasi Gagal</h2><p>Kode otorisasi tidak ditemukan.</p>",
            Err(ServerError::Auth(AuthError::MissingCode))
        ),
    };

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    result_code
}

pub fn listen_for_code() -> Result<String, ServerError> {
    start_server()
}