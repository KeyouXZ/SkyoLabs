use crate::web::net::AP_IP;
use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use edge_dhcp::{
    io::{self, DEFAULT_SERVER_PORT},
    server::{Server, ServerOptions},
};
use edge_nal::UdpBind;
use edge_nal_embassy::{Udp, UdpBuffers};
use embassy_net::Stack;

#[embassy_executor::task]
pub async fn dhcp_server(stack: Stack<'static>) {
    let buffers = UdpBuffers::<3, 1024, 1024, 10>::new();
    let udp = Udp::new(stack, &buffers);

    let address = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::UNSPECIFIED,
        DEFAULT_SERVER_PORT,
    ));

    let mut socket = match udp.bind(address).await {
        Ok(socket) => socket,
        Err(error) => {
            log::error!("DHCP bind failed: {:?}", error);
            return;
        }
    };

    let mut packet_buffer = [0u8; 1500];
    let mut gateway_buffer = [Ipv4Addr::UNSPECIFIED];
    let dns_servers = [AP_IP];

    let mut server = Server::<_, 64>::new_with_et(AP_IP);
    let mut options = ServerOptions::new(AP_IP, Some(&mut gateway_buffer));

    options.dns = &dns_servers;
    options.captive_url = Some("http://192.168.4.1/");

    log::info!("DHCP server listening");
    log::info!("DHCP DNS server: {}", AP_IP);

    loop {
        if let Err(error) =
            io::server::run(&mut server, &options, &mut socket, &mut packet_buffer).await
        {
            log::warn!("DHCP server error: {:?}", error);
            embassy_time::Timer::after_millis(500).await;
        }
    }
}
