use crate::web::net::AP_IP;
use core::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use edge_captive::io::run;
use edge_nal_embassy::{Udp, UdpBuffers};
use embassy_net::Stack;

#[embassy_executor::task]
pub async fn dns_server(stack: Stack<'static>) {
    let buffers = UdpBuffers::<3, 1024, 1024, 10>::new();
    let udp = Udp::new(stack, &buffers);

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 53);

    let mut tx_buffer = [0u8; 1500];
    let mut rx_buffer = [0u8; 1500];

    loop {
        log::info!("Captive DNS listening on UDP port 53");
        if let Err(error) = run(
            &udp,
            address,
            &mut tx_buffer,
            &mut rx_buffer,
            AP_IP,
            Duration::from_secs(60),
        )
        .await
        {
            log::warn!("Captive DNS error: {:?}", error);
            embassy_time::Timer::after_millis(500).await;
        }
    }
}
