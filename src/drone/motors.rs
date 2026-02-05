use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use rtt_target::rprintln as println;
use stm32f4xx_hal::{
    interrupt,
    pac::{self, Interrupt, TIM2},
    prelude::*,
    rcc::Rcc,
    timer::{CounterHz, Event},
};

#[derive(Clone, Copy, Debug)]
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

static G_TIM: Mutex<RefCell<Option<CounterHz<TIM2>>>> = Mutex::new(RefCell::new(None));

pub fn setup(timer: pac::TIM2, rcc: &mut Rcc) {
    let mut timer: CounterHz<pac::TIM2> = timer.counter_hz(rcc);
    timer.start(1.Hz()).unwrap();
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| {
        G_TIM.borrow(cs).replace(Some(timer));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }
}

pub fn set_intent(intent: Intent) {
    cortex_m::interrupt::free(|cs| {
        G_INTENT.borrow(cs).replace(intent);
    });
}

pub fn get_intent() -> Intent {
    cortex_m::interrupt::free(|cs| *G_INTENT.borrow(cs).borrow())
}

#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterHz<TIM2>> = None;
    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    let intent = get_intent();
    println!("{:?}", intent);
    let _ = tim.wait();
}
