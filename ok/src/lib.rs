
#![no_std]

use defmt_rtt as _;
use heapless::{String, Vec};
use heapless::consts::*;

use stm32f4xx_hal as hal;
#[allow(unused_imports)]
use hal::{prelude::*, stm32, serial, timer, delay, interrupt};
#[allow(unused_imports)]
use hal::gpio::{Edge, ExtiPin, gpioa, gpiob, gpioc, gpiof, Alternate, Input, Output, PushPull, PullUp, PullDown};
use cortex_m::interrupt::{free, Mutex};
use core::cell::Cell;
use core::cell::RefCell;
use core::ops::DerefMut;
use core::fmt::Write;

pub static LAST_LOG_TIMER: Mutex<Cell<u64>> = Mutex::new(Cell::new(0));
pub static LOG_TIMER: Mutex<Cell<u64>> = Mutex::new(Cell::new(0));

/// USART1 for debuging
pub static SER1: Mutex<RefCell<Option<
        serial::Serial<
                stm32::USART1,
            (
                gpioa::PA9<Alternate<hal::gpio::AF7>>, // TX
                gpioa::PA10<Alternate<hal::gpio::AF7>>, // RX
            ),
            >,
    >>> = Mutex::new(RefCell::new(None));

/// USART2 for debuging
pub static SER2: Mutex<RefCell<Option<
        serial::Serial<
                stm32::USART2,
            (
                gpioa::PA2<Alternate<hal::gpio::AF7>>, // TX
                gpioa::PA3<Alternate<hal::gpio::AF7>>, // RX
            ),
            >,
    >>> = Mutex::new(RefCell::new(None));

/// USART3 for debuging
pub static SER3: Mutex<RefCell<Option<
        serial::Serial<
                stm32::USART3,
            (
                gpiob::PB10<Alternate<hal::gpio::AF7>>, // TX
                gpiob::PB11<Alternate<hal::gpio::AF7>>, // RX
            ),
            >,
    >>> = Mutex::new(RefCell::new(None));

/// USART6 for debuging
pub static SER6: Mutex<RefCell<Option<
        serial::Serial<
                stm32::USART6,
            (
                gpioc::PC6<Alternate<hal::gpio::AF8>>, // TX
                gpioc::PC7<Alternate<hal::gpio::AF8>>, // RX
            ),
            >,
    >>> = Mutex::new(RefCell::new(None));

/// GPIO: LED_RED
pub static LED_RED: Mutex<RefCell<Option<gpiof::PF9<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

/// GPIO: LED_GREEN
pub static LED_GREEN: Mutex<RefCell<Option<gpiof::PF10<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

/// TM2
pub static TM2: Mutex<RefCell<Option<timer::Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));

/// TM3
pub static TM3: Mutex<RefCell<Option<timer::Timer<stm32::TIM3>>>> = Mutex::new(RefCell::new(None));

pub static COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

pub fn usart1_log (maybe_string: String<U100>) {
    free(|cs| {
        if let Some(ref mut ser1) = SER1.borrow(cs).borrow_mut().deref_mut() {
            for a in maybe_string.into_bytes() {
                nb::block!(ser1.write(a)).ok();
            }
        };
    })
}

