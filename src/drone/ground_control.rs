use core::cell::{Cell, RefCell};
use cortex_m::interrupt::Mutex;
use panic_halt as _;
use stm32f4xx_hal::{
    interrupt,
    pac::{self},
    prelude::*,
    serial::{self, Serial},
};

type SerialType = Serial<pac::USART2>;
const START_BYTE: u8 = 0xAA; // this char -> ª
static G_SERIAL: Mutex<RefCell<Option<SerialType>>> = Mutex::new(RefCell::new(None));
const FRAME_LEN: usize = 6;
static G_GROUND_CONTROL: Mutex<RefCell<ControlFrame>> = Mutex::new(RefCell::new([0; FRAME_LEN]));
static G_GROUND_CONTROL_FRAME_READY: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

pub type ControlFrame = [u8; FRAME_LEN];

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

pub fn take_frame() -> Option<ControlFrame> {
    cortex_m::interrupt::free(|cs| {
        if G_GROUND_CONTROL_FRAME_READY.borrow(cs).get() {
            G_GROUND_CONTROL_FRAME_READY.borrow(cs).set(false);
            Some(*G_GROUND_CONTROL.borrow(cs).borrow())
        } else {
            None
        }
    })
}

#[interrupt]
fn USART2() {
    static mut SERIAL: Option<SerialType> = None;
    static mut GROUND_CONTROL_IN: ControlFrame = [0; FRAME_LEN];
    static mut CURSOR: usize = 0;

    let serial = SERIAL.get_or_insert_with(|| {
        // replace global G_SERIAL with None and put serial into local SERIAL.
        // this allows exlusive ownership of the serial peripheral by this ISR.
        cortex_m::interrupt::free(|cs| G_SERIAL.borrow(cs).replace(None).unwrap())
    });

    if let Ok(byte) = serial.read() {
        // we are not aligned, ignore bytes until START_BYTE
        if !(*CURSOR == 0 && byte != START_BYTE) {
            if byte == START_BYTE {
                *CURSOR = 0
            }

            GROUND_CONTROL_IN[*CURSOR] = byte;
            *CURSOR += 1;

            if *CURSOR == FRAME_LEN {
                // we have a whole frame, copy to global buffer inside critical section
                cortex_m::interrupt::free(|cs| {
                    G_GROUND_CONTROL
                        .borrow(cs)
                        .borrow_mut()
                        .copy_from_slice(GROUND_CONTROL_IN);

                    G_GROUND_CONTROL_FRAME_READY.borrow(cs).set(true);
                });

                *CURSOR = 0
            }
        }
    }
}
