#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, radio};
use embassy_time::Timer;
use jewel::{
    gap::{AdvData, Flags},
    ll::{Address, AddressAndData, AdvNonconnInd},
    phy::{BleRadio, MAX_PDU_LENGTH},
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    RADIO => radio::InterruptHandler<peripherals::RADIO>;
});

// Same payload as the embassy/nrf-softdevice ble_advertising example,
// but just in channel 39.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    info!("Starting BLE radio");
    let mut radio = radio::ble::Radio::new(p.RADIO, Irqs);

    let body = AdvData::empty()
        .set_flags(Flags::discoverable())
        .set_uuids16(&[0x0918])
        .set_complete_local_name("HelloRust");

    let mut body_buffer = [0u8; MAX_PDU_LENGTH];
    let len = body.bytes(&mut body_buffer);

    let pdu = AdvNonconnInd::new(Address::new_random(0xffe1e8d0dc27), &body_buffer[..len]);

    let mut pdu_buffer = [0u8; MAX_PDU_LENGTH];
    pdu.bytes(&mut pdu_buffer);

    unwrap!(radio.set_buffer(&pdu_buffer));

    loop {
        info!("Sending packet");
        radio.transmit().await;
        Timer::after_millis(500).await;
    }
}
