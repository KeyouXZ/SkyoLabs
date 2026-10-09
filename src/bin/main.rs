#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use log::{error, info};
use schoolesp::{se, web};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    // Start TIMER
    info!("Starting timer....");
    web::wifi::start_timer(peripherals.TIMG0, peripherals.FROM_CPU_INTR0);

    // Connect to WIfi
    info!("Starting Wi-Fi...");
    let _w_ctrl = web::wifi::connect_wifi(peripherals.WIFI);

    info!("Starting core 1...");
    web::exec::start_second_core(peripherals.CPU_CTRL, peripherals.FROM_CPU_INTR1);

    #[cfg(feature = "dummy")]
    loop {
        log::info!("Dummy listening!");
        let delay_start = esp_hal::time::Instant::now();
        while delay_start.elapsed() < esp_hal::time::Duration::from_millis(500) {}
    }

    #[cfg(not(feature = "dummy"))]
    // Define pins
    let pins = se::core::Pins {
        // MQ-7 Pin 4
        mq7: peripherals.GPIO4,
        // MQ-7 ADC 2 cuz of pin 4 is ADC-capable 2
        mq7_adc: peripherals.ADC2,

        // SSD1306 I2C
        ssd1306_i2c: peripherals.I2C0,
        // SSD1306 SDA Pin 22
        ssd1306_sda: peripherals.GPIO22.into(),
        // SSD1306 SCL Pin 23
        ssd1306_scl: peripherals.GPIO23.into(),
        // DHT22 Pin 5
        dht22: peripherals.GPIO5.into(),
        // Buzzer Pin 15
        buzzer: peripherals.GPIO15.into(),
    };

    #[cfg(not(feature = "dummy"))]
    se::main::start(pins);
}
