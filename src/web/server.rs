use crate::se::shared::get_sensors;
use embassy_net::Stack;
use picoserve::{
    AppBuilder, AppRouter, Router,
    response::{Json, Response},
    routing::get,
};
use serde::Serialize;

// const HTML: &str = r#"<!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta charset="UTF-8">
//     <meta name="viewport" content="width=device-width, initial-scale=1">
//     <title>SkyoLabs</title>
//     <style>
//         body {
//             margin: 0;
//             min-height: 100vh;
//             display: grid;
//             place-items: center;
//             background: #101827;
//             color: #e5e7eb;
//             font-family: system-ui, sans-serif;
//         }
//         main { text-align: center; padding: 2rem; }
//         h1 { color: #60a5fa; }
//         p { color: #9ca3af; }
//         button {
//             padding: 0.7rem 1.2rem;
//             border: 0;
//             border-radius: 8px;
//             background: #2563eb;
//             color: white;
//         }
//         pre {
//             text-align: left;
//             background: #1e293b;
//             padding: 1rem;
//             border-radius: 8px;
//         }
//     </style>
// </head>
// <body>
//     <main>
//         <h1>SkyoLabs</h1>
//         <p>ESP32 captive portal is running.</p>
//         <button onclick="getStatus()">Refresh status</button>
//         <pre id="status">Loading...</pre>
//     </main>
//     <script>
//     async function getSensors() {
//             const output = document.getElementById("status");
//
//             try {
//                 const response = await fetch("/api/sensors");
//
//                 if (!response.ok) {
//                     throw new Error(`HTTP ${response.status}`);
//                 }
//
//                 const sensors = await response.json();
//                 output.textContent = JSON.stringify(sensors, null, 2);
//             } catch (error) {
//                 output.textContent = `Connection error: ${error}`;
//             }
//         }
//
//         getSensors();
//         setInterval(getSensors, 1000);
//     </script>
// </body>
// </html>
// "#;

const HTML: &str = include_str!("../../web/dist/index.html");
const FAVICON: &str = include_str!("../../web/dist/favicon.svg");

#[derive(Serialize)]
struct DeviceStatus {
    device: &'static str,
    status: &'static str,
    ip: &'static str,
}

struct SkyoLabsApp;

impl AppBuilder for SkyoLabsApp {
    type PathRouter = impl picoserve::routing::PathRouter;

    fn build_app(self) -> Router<Self::PathRouter> {
        Router::new()
            .route(
                "/",
                get(|| async { Response::ok(HTML).with_content_type("text/html; charset=utf-8") }),
            )
            .route(
                "/favicon.svg",
                get(|| async {
                    Response::ok(FAVICON).with_content_type("text/html; charset=utf-8")
                }),
            )
            .route(
                "/api/status",
                get(|| async {
                    Json(DeviceStatus {
                        device: "SkyoLabs",
                        status: "online",
                        ip: "192.168.4.1",
                    })
                }),
            )
            .route("/api/sensors", get(|| async { Json(get_sensors()) }))
            .route(
                "/generate_204",
                get(|| async { Response::ok(HTML).with_content_type("text/html; charset=utf-8") }),
            )
            .route(
                "/hotspot-detect.html",
                get(|| async { Response::ok(HTML).with_content_type("text/html; charset=utf-8") }),
            )
            .route(
                "/connecttest.txt",
                get(|| async { Response::ok(HTML).with_content_type("text/html; charset=utf-8") }),
            )
            .route(
                "/ncsi.txt",
                get(|| async { Response::ok(HTML).with_content_type("text/html; charset=utf-8") }),
            )
    }
}

static CONFIG: picoserve::Config = picoserve::Config::const_default().keep_connection_alive();

#[embassy_executor::task]
pub async fn http_server(stack: Stack<'static>) -> ! {
    let app = picoserve::make_static!(AppRouter<SkyoLabsApp>, SkyoLabsApp.build_app());

    let mut tcp_rx_buffer = [0u8; 1024];
    let mut tcp_tx_buffer = [0u8; 1024];
    let mut http_buffer = [0u8; 2048];

    log::info!("HTTP: picoserve listening on port 80");

    picoserve::Server::new(app, &CONFIG, &mut http_buffer)
        .listen_and_serve(0, stack, 80, &mut tcp_rx_buffer, &mut tcp_tx_buffer)
        .await
        .into_never()
}
