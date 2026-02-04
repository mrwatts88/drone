#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln as println, rtt_init_print};
use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::Config,
    serial::{self, Serial},
};

mod drone;
use drone::ground_control;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    println!("Running...");

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));
    let gpioa = dp.GPIOA.split(&mut rcc);

    let tx = gpioa.pa2;
    let rx = gpioa.pa3;

    let serial: Serial<pac::USART2> = Serial::new(
        dp.USART2,
        (tx, rx),
        serial::Config::default().baudrate(115200.bps()),
        &mut rcc,
    )
    .unwrap();

    ground_control::setup(serial);

    #[allow(clippy::empty_loop)]
    loop {}
}
