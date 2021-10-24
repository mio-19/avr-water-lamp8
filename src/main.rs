#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

trait Led {
    fn on();
    fn off();
}

impl <T: Pin> Led for T {
    fn on() {
        Self::set_high();
    }
    fn off() {
        Self::set_low();
    }
}

fn all_on() {
    port::B0::on();
    port::B1::on();
    port::B2::on();
    port::B3::on();
     port::B4::on();
    port::B5::on();
    port::B6::on();
    port::B7::on();
}

fn delay(){
        ruduino::delay::delay_ms(1000);
}

#[no_mangle]
pub extern fn main() {
    port::B0::set_output();
    port::B1::set_output();
    port::B2::set_output();
    port::B3::set_output();
    port::B4::set_output();
    port::B5::set_output();
    port::B6::set_output();
    port::B7::set_output();

    loop {
        all_on();
        port::B0::off();

        delay();

        all_on();
        port::B1::off();

        delay();

        all_on();
        port::B2::off();

        delay();

        all_on();
        port::B3::off();

        delay();

        all_on();
        port::B4::off();

        delay();

        all_on();
        port::B5::off();

        delay();

        all_on();
        port::B6::off();

        delay();

        all_on();
        port::B7::off();

        delay();
    }
}
