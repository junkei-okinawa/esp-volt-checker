use log::info;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::adc::{
    attenuation::DB_11,
    oneshot::AdcDriver,
    oneshot::AdcChannelDriver,
    oneshot::config::{AdcChannelConfig, Calibration},
};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let adc2 = AdcDriver::new(peripherals.adc2)?;

    // configuring pin to analog read, you can regulate the adc input voltage range depending on your need
    // for this example we use the attenuation of 11db which sets the input voltage range to around 0-3.6V
    let config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: Calibration::Line,
        ..Default::default()
    };
    let mut adc2_ch1 = AdcChannelDriver::new(&adc2, peripherals.pins.gpio0, &config)?;

    const MIN_MV: f32 = 128.0;
    const MAX_MV: f32 = 3130.0;
    const RANGE_MV: f32 = MAX_MV - MIN_MV;

    loop {
        // you can change the sleep duration depending on how often you want to sample
        FreeRtos::delay_ms(100);
        let voltage_mv = adc2_ch1.read()? as f32; // Cast to f32 for calculation

        // Calculate percentage within the observed range [MIN_MV, MAX_MV]
        let percentage = if RANGE_MV <= 0.0 {
            0.0 // Avoid division by zero or negative range
        } else {
            ((voltage_mv - MIN_MV) / RANGE_MV * 100.0).max(0.0).min(100.0) // Clamp between 0 and 100
        };

        info!("Voltage: {:.0} mV ({:.1}%)", voltage_mv, percentage);
    }
}
