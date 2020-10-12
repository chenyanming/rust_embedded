#![no_main]
#![no_std]

// Halt on panic
// #[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
// extern crate panic_halt; // panic handler
// extern crate panic_itm; // panic handler

// use panic_halt as _;
// use cortex_m_rt::entry;
#[allow(unused_imports)]
use cortex_m_semihosting::{debug, hprintln};

use stm32f4xx_hal as hal;
#[allow(unused_imports)]
use hal::{
    prelude::*, stm32, serial, timer, delay, interrupt,
    gpio::{Edge, ExtiPin, gpioa, gpiob, gpioc, gpiof, Alternate, Input, Output, PushPull, PullUp, PullDown},
};

// use rtt_target::{rtt_init_print, rprintln};
use defmt_rtt as _;
use panic_probe as _;

#[allow(unused_imports)]
use heapless::{
    String,
    consts::*,
    i,
    spsc::{Consumer, Producer, Queue},
    Vec,
};

#[allow(unused_imports)]
use core::{
    cell::{Cell, RefCell},
    ops::DerefMut,
    sync::atomic::{AtomicUsize, Ordering},
    fmt::Write,
};

#[allow(unused_imports)]
use cortex_m::{
    self,
    interrupt::{free, Mutex},
    peripheral::DWT,
};
// use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

#[allow(unused_imports)]
use rtic::cyccnt::{Instant, U32Ext};


#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {

};
