#![no_std]
#![no_main]

use cortex_m;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use stm32f4xx_hal as hal;
use hal::{prelude::*, stm32, serial, timer, delay, interrupt};
use hal::gpio::{gpioa, gpiof, Alternate, Output, PushPull};

#[allow(unused_imports)]
use heapless::{
    String,
    consts::*,
    i,
    spsc::{Consumer, Producer, Queue},
    Vec,
};

use panic_probe as _;

use ok;
// pub mod vccs;
use ok::vccs::{Package, FujiPacketErr};

#[entry]
fn main() -> ! {

    test_cool_function();

    // test vccs functions
    let vccs_package = Package {..Default::default()};

    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    let ga = dp.GPIOA.split();

    dp.USART1.cr1.modify(|_,w|{
        w.rxneie().set_bit()
    });
    let (mut tx, mut rx) = serial::Serial::usart1(
        dp.USART1,
        (ga.pa9.into_alternate_af7(), ga.pa10.into_alternate_af7()),
        serial::config::Config::default().baudrate(115_200.bps()),
        clocks,
    ).unwrap().split();
    loop {
        let d1: Result<u8,stm32f4xx_hal::nb::Error<stm32f4xx_hal::serial::Error>> = rx.read();
        match d1 {
            Ok(T) => {
                defmt::info!("{:u8}", T);
                assert!(T > 128, "Input {} larger than 128", T);
            }
            Err(E) => {
               
            }

        }
    }
    // {{crate_name}}::exit();
}

fn test_cool_function() {
    let mut cool = ok::CoolStruct {
        x: 100,
        y: 2000,
    };
    let p_cool = &mut cool as *mut ok::CoolStruct;
    unsafe {
        let test_cool = ok::cool_functon(3, 1, p_cool);
        defmt::info!("Called cool_function:{:i32}", test_cool);
        assert!(test_cool == 100, "Called cool_function:{}", test_cool);

    }

}

fn test_checksum(vccs_package: Package) {

    let mut d: Vec<u8, U13> = Vec::new();
    d.push(0x02);
    d.push(0x00);
    d.push(0x85);
    let crc: u8 = vccs_package.checksum(&d);
    assert!(crc == 0x87, "Called vccs checksum:{}", crc);
}
