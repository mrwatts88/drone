use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use rtt_target::rprintln as println;
use stm32f4xx_hal::{
    hal_02::PwmPin,
    interrupt,
    pac::{self, Interrupt, TIM2},
    prelude::*,
    rcc::Rcc,
    timer::{CounterHz, Event},
};

pub const ESC_PERIOD_US: u16 = 2500;

#[derive(Clone, Copy, Debug)]
pub struct Intent {
    pub roll: u8,
    pub pitch: u8,
    pub yaw: u8,
    pub throttle: u8,
}

static G_INTENT: Mutex<RefCell<Intent>> = Mutex::new(RefCell::new(Intent {
    roll: 0,
    pitch: 0,
    yaw: 0,
    throttle: 0,
}));

static G_MOTOR_VALUES: Mutex<RefCell<[u16; 4]>> = Mutex::new(RefCell::new([0; 4]));
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

pub fn update_esc_duty<
    P1: PwmPin<Duty = u16>,
    P2: PwmPin<Duty = u16>,
    P3: PwmPin<Duty = u16>,
    P4: PwmPin<Duty = u16>,
>(
    ch1: &mut P1,
    ch2: &mut P2,
    ch3: &mut P3,
    ch4: &mut P4,
) {
    let motor_values = cortex_m::interrupt::free(|cs| *G_MOTOR_VALUES.borrow(cs).borrow());
    set_esc_duty(ch1, motor_values[0]);
    set_esc_duty(ch2, motor_values[1]);
    set_esc_duty(ch3, motor_values[2]);
    set_esc_duty(ch4, motor_values[3]);
}

fn set_esc_duty<P: PwmPin<Duty = u16>>(ch: &mut P, pulse_len_us: u16) {
    let max_duty = ch.get_max_duty();
    let ccr = (pulse_len_us as u32 * max_duty as u32 / ESC_PERIOD_US as u32) as u16;
    ch.set_duty(ccr.min(max_duty));
}

#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterHz<TIM2>> = None;
    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    let intent = cortex_m::interrupt::free(|cs| *G_INTENT.borrow(cs).borrow());
    println!("{:?}", intent);

    // pid calc
    // motor mixing

    cortex_m::interrupt::free(|cs| {
        G_MOTOR_VALUES.borrow(cs).replace([1000, 1000, 1000, 10]);
    });

    let _ = tim.wait();
}
