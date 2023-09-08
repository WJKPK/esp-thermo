use hal::{
    gpio::{NO_PIN, OutputPin},
    peripherals::SPI2,
    spi::{HalfDuplexMode, SpiDataMode, Error, SpiMode},
    spi::master::{Spi, HalfDuplexReadWrite, Command, Address},
    prelude::_fugit_RateExtU32, 
    clock::Clocks,
}; 

use esp_println::println;
use crate::hardware_concierge::bsp::*;

pub struct ThermoControl {
    spi: Spi<'static, SPI2, HalfDuplexMode>,
    toggler: HeaterToggler
}

impl ThermoControl {
        pub fn new(
        spi: ThermocoupleAfeSpi,
        sclk: impl Into<ThermocoupleAfeSclk>,
        miso: impl Into<ThermocoupleAfeMiso>,
        cs: impl Into<ThermocoupleAfeCs>,
        toggler: impl Into<HeaterToggler>,
        clocks: &Clocks,
    ) -> Self {
        let hd_spi = Spi::new_half_duplex(
            spi,
            Some(sclk.into()),
            NO_PIN,
            Some(miso.into()),
            NO_PIN,
            NO_PIN,
            Some(cs.into()),
            4u32.MHz(),
            SpiMode::Mode0,
            clocks
        );

        ThermoControl {
            spi: hd_spi,
            toggler: toggler.into()
        }
    }

    pub fn read_temperature(&mut self) -> Result<u16, Error> {
        let mut read_buffer: [u8; 2] = [0, 0];
        self.spi.read(
            SpiDataMode::Single,
            Command::None,
            Address::None,
            0,
            &mut read_buffer 
        )?;
        
        let casted_temp = u16::from_be_bytes(read_buffer);
        if ((casted_temp & 0x4) >> 2) == 1 {
            println!("Error during thermopare readout!");
            return Err(Error::Unknown);
        }
        Ok(((casted_temp & 0x7FF8) >> 3)/ 4)
    }

    pub fn set_heater_state(&mut self, state: bool) {
        let _pin = self.toggler.set_output_high(state);
    }
}

