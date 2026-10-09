use core::net::Ipv4Addr;

use embassy_net::{Config, Ipv4Cidr, Stack, StackResources, StaticConfigV4};

use esp_radio::wifi::Interface;
use static_cell::StaticCell;

static STACK_RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();

pub const AP_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 4, 1);

pub fn create_stack(seed: u64) -> (Stack<'static>, embassy_net::Runner<'static, Interface>) {
    let interface = Interface::access_point();

    let config = Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(AP_IP, 24),
        gateway: Some(AP_IP),
        dns_servers: Default::default(),
    });

    embassy_net::new(
        interface,
        config,
        STACK_RESOURCES.init(StackResources::new()),
        seed,
    )
}

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface>) -> ! {
    runner.run().await
}
