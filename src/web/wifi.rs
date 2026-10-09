use crate::se;
use esp_hal::peripherals;
use esp_radio::wifi::{
    Config::AccessPoint, ControllerConfig, Ssid, WifiController, ap::AccessPointConfig,
};

pub fn start_timer(
    timg0: peripherals::TIMG0<'static>,
    intr0: peripherals::FROM_CPU_INTR0<'static>,
) {
    let timg0 = esp_hal::timer::timg::TimerGroup::new(timg0);
    esp_rtos::start(timg0.timer0, intr0);
}

pub fn connect_wifi<'a>(wifi: esp_hal::peripherals::WIFI<'a>) -> WifiController<'a> {
    let ssid = match Ssid::try_from(se::core::SSID) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    let ap_config = AccessPointConfig::default().with_ssid(ssid);
    let w_config = ControllerConfig::default().with_initial_config(AccessPoint(ap_config));

    let w_ctrl = match WifiController::new(wifi, w_config) {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e);
        }
    };

    w_ctrl
}
