use core::fmt::Write as FmtWrite;

use embassy_net::{Stack, tcp::TcpSocket};
use embedded_io_async::Write as _;
use heapless::String;

const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>SkyoLabs</title>
    <style>
        body {
            margin: 0;
            min-height: 100vh;
            display: grid;
            place-items: center;
            background: #101827;
            color: #e5e7eb;
            font-family: system-ui, sans-serif;
        }
        main {
            text-align: center;
            padding: 2rem;
        }
        h1 { color: #60a5fa; }
        p { color: #9ca3af; }
    </style>
</head>
<body>
    <main>
        <h1>SkyoLabs</h1>
        <p>ESP32 captive portal is running.</p>
    </main>
</body>
</html>
"#;

const RESPONSE_CAPACITY: usize = 2048;
const SOCKET_BUFFER_SIZE: usize = 2048;
const REQUEST_BUFFER_SIZE: usize = 1024;

#[embassy_executor::task]
pub async fn http_server(stack: Stack<'static>) -> ! {
    let mut rx_buffer = [0u8; SOCKET_BUFFER_SIZE];
    let mut tx_buffer = [0u8; SOCKET_BUFFER_SIZE];
    let mut request_buffer = [0u8; REQUEST_BUFFER_SIZE];

    loop {
        log::info!("HTTP: waiting for client");

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        if let Err(error) = socket.accept(80).await {
            log::warn!("HTTP: accept failed: {:?}", error);
            socket.abort();
            embassy_time::Timer::after_millis(250).await;
            continue;
        }

        log::info!("HTTP: client connected");

        let request_len = match socket.read(&mut request_buffer).await {
            Ok(0) => {
                log::info!("HTTP: client disconnected without request");
                socket.abort();
                continue;
            }
            Ok(len) => {
                log::info!("HTTP: received {} bytes", len);
                len
            }
            Err(error) => {
                log::warn!("HTTP: read failed: {:?}", error);
                socket.abort();
                continue;
            }
        };

        if let Ok(request) = core::str::from_utf8(&request_buffer[..request_len]) {
            if let Some(first_line) = request.lines().next() {
                log::info!("HTTP: request: {}", first_line);
            }
        }

        let mut response: String<RESPONSE_CAPACITY> = String::new();

        let result = write!(
            response,
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=utf-8\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             Cache-Control: no-store\r\n\
             \r\n\
             {}",
            HTML.len(),
            HTML,
        );

        if result.is_err() {
            log::error!("HTTP: failed to build response");
            socket.abort();
            continue;
        }

        log::info!("HTTP: sending response ({} bytes)", response.len());

        match socket.write_all(response.as_bytes()).await {
            Ok(()) => log::info!("HTTP: response written"),
            Err(error) => {
                log::error!("HTTP: write failed: {:?}", error);
                socket.abort();
                continue;
            }
        }

        match socket.flush().await {
            Ok(()) => log::info!("HTTP: flush complete"),
            Err(error) => {
                log::warn!("HTTP: flush failed: {:?}", error);
            }
        }

        socket.close();
        log::info!("HTTP: response complete, connection closing");
    }
}
