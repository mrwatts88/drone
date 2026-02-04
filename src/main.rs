#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    self as hal, interrupt,
    rcc::Config,
    serial::{self, Serial},
};

use crate::hal::{pac, prelude::*};

type SerialType = Serial<pac::USART2>;
static SERIAL: Mutex<RefCell<Option<SerialType>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("hello from the MCU");

    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock at 48MHz
        let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));

        // Set up GPIO A (USART2 TX=PA2, RX=PA3 - connected to ST-LINK USB)
        let gpioa = dp.GPIOA.split(&mut rcc);

        // Configure USART2 pins
        let tx = gpioa.pa2.into_alternate();
        let rx = gpioa.pa3.into_alternate();

        // Set up USART2 at 115200 baud
        let mut serial = Serial::new(
            dp.USART2,
            (tx, rx),
            serial::Config::default().baudrate(115200.bps()),
            &mut rcc,
        )
        .unwrap();

        // Enable RX interrupt
        serial.listen(serial::Event::RxNotEmpty);

        // Store serial in static for interrupt access
        cortex_m::interrupt::free(|cs| {
            SERIAL.borrow(cs).replace(Some(serial));
        });

        // Enable USART2 interrupt in NVIC
        unsafe {
            cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART2);
        }

        rprintln!("UART ready on PA2/PA3 (ST-LINK USB). Send data!");
    }

    loop {}
}

#[interrupt]
fn USART2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(serial) = SERIAL.borrow(cs).borrow_mut().as_mut()
            && let Ok(byte) = serial.read()
        {
            rprintln!("Received: {} ('{}')", byte, byte as char);
        }
    });
}
