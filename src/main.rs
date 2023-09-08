#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

mod hardware_concierge;
mod device_logic;

use esp_backtrace as _;
use esp_println::println;
use hal::{embassy, clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, IO};
use esp_wifi::{initialize, EspWifiInitFor};
use hal::{systimer::SystemTimer, Rng};
use crate::device_logic::ble;
use crate::hardware_concierge::thermo_control::ThermoControl;
use crate::device_logic::heater_controller;

#[main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    esp_println::logger::init_logger(log::LevelFilter::Info);
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let init = initialize(
        EspWifiInitFor::Ble,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio10;
    let cs = io.pins.gpio5;
    let miso = io.pins.gpio4;
    let toggler = io.pins.gpio8.into_push_pull_output();

    // Async requires the GPIO interrupt to wake futures
    hal::interrupt::enable(
        hal::peripherals::Interrupt::GPIO,
        hal::interrupt::Priority::Priority1,
    )
    .unwrap();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group0.timer0);

    let bluetooth = peripherals.BT;

    let thermo_control = ThermoControl::new(peripherals.SPI2, sclk, miso, cs, toggler, &clocks);

    spawner.spawn(heater_controller::run(thermo_control)).unwrap();
    spawner.spawn(ble::run(init, bluetooth)).unwrap();
    println!("Spawned!");
}
