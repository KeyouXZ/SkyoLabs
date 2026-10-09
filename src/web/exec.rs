use crate::{
    se::core::{CORE1_EXECUTOR, CORE1_STACK},
    web::{
        dhcp::dhcp_server,
        dns::dns_server,
        net::{create_stack, net_task},
        server::http_server,
    },
};
use esp_hal::peripherals::{CPU_CTRL, FROM_CPU_INTR1};

pub fn start_second_core(cpu_ctrl: CPU_CTRL, intr1: FROM_CPU_INTR1<'static>) {
    esp_rtos::start_second_core(cpu_ctrl, intr1, CORE1_STACK.take(), || {
        let executor = CORE1_EXECUTOR.init(esp_rtos::embassy::Executor::new());

        let seed = u64::from(esp_hal::rng::Rng::new().random());
        let (stack, runner) = create_stack(seed);

        executor.run(|spawner| {
            spawner.spawn(net_task(runner).unwrap());

            spawner.spawn(dhcp_server(stack).unwrap());
            spawner.spawn(dns_server(stack).unwrap());
            spawner.spawn(http_server(stack).unwrap());

            spawner.spawn(wifi_task().unwrap());
        });
    });
}

#[embassy_executor::task]
async fn wifi_task() {
    loop {
        log::info!("Wi-Fi task is alive!");

        embassy_time::Timer::after_secs(1).await;
    }
}
