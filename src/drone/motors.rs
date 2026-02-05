use core::cell::RefCell;

use cortex_m::interrupt::Mutex;

#[derive(Clone, Copy)]
pub struct Intent {
    pub roll: u8,
    pub pitch: u8,
    pub yaw: u8,
    pub throttle: u8,
}

pub static G_INTENT: Mutex<RefCell<Intent>> = Mutex::new(RefCell::new(Intent {
    roll: 0,
    pitch: 0,
    yaw: 0,
    throttle: 0,
}));

pub fn set_intent(intent: Intent) {
    cortex_m::interrupt::free(|cs| {
        G_INTENT.borrow(cs).replace(intent);
    });
}

pub fn get_intent() -> Intent {
    cortex_m::interrupt::free(|cs| *G_INTENT.borrow(cs).borrow())
}
