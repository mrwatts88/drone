use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use panic_halt as _;
use rtt_target::rprintln;
use stm32f4xx_hal::{
    interrupt, pac,
    prelude::*,
    serial::{self, Serial},
};

type SerialType = Serial<pac::USART2>;
static G_SERIAL: Mutex<RefCell<Option<SerialType>>> = Mutex::new(RefCell::new(None));

pub fn setup(mut serial: SerialType) {
    serial.listen(serial::Event::RxNotEmpty);

    cortex_m::interrupt::free(|cs| {
        G_SERIAL.borrow(cs).replace(Some(serial));
    });

    // Enable USART2 interrupt
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::USART2);
    }
}

#[interrupt]
fn USART2() {
    // move Serial into local static var without mutex refcell
    static mut SERIAL: Option<SerialType> = None;

    let serial = SERIAL.get_or_insert_with(|| {
        // replace global G_SERIAL with None and put serial into local SERIAL.
        // this allows exlusive ownership of the serial peripheral by this ISR.
        cortex_m::interrupt::free(|cs| G_SERIAL.borrow(cs).replace(None).unwrap())
    });

    if let Ok(byte) = serial.read() {
        // print incoming byte
        rprintln!("Received: {} ('{}')", byte, byte as char);

        // echo byte
        serial.write(byte).unwrap();
    }
}
