use hal::{
    gpio::{Floating, GpioPin, Input, Output, PushPull},
    peripherals::SPI2,
};

pub type ThermocoupleAfeSpi = SPI2;
pub type ThermocoupleAfeSclk = GpioPin<Output<PushPull>, 10>;
pub type ThermocoupleAfeCs = GpioPin<Output<PushPull>, 5>;
pub type ThermocoupleAfeMiso = GpioPin<Input<Floating>, 4>;
pub type HeaterToggler = GpioPin<Output<PushPull>, 8>;
