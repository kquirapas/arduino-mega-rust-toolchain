#![no_std]
#![no_main]

use arduino_hal::{
    delay_ms,
    simple_pwm::{IntoPwmPin, Timer3Pwm},
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut duty_cycle = 10;
    let timer = Timer3Pwm::new(dp.TC3, arduino_hal::simple_pwm::Prescaler::Direct);
    let mut blue_led_pwm = pins.d2.into_output().into_pwm(&timer);
    let mut is_brightening = true;
    blue_led_pwm.set_duty(duty_cycle);
    blue_led_pwm.enable();

    loop {
        delay_ms(4);
        if is_brightening {
            blue_led_pwm.set_duty(duty_cycle);
            duty_cycle = duty_cycle.saturating_add(1);
            if duty_cycle == u8::MAX {
                is_brightening = !is_brightening;
            }
        } else {
            blue_led_pwm.set_duty(duty_cycle);
            duty_cycle = duty_cycle.saturating_sub(1);
            if duty_cycle == u8::MIN {
                is_brightening = !is_brightening;
            }
        }
    }
}
