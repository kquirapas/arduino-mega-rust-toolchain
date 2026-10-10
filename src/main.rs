#![no_std]
#![no_main]

use arduino_hal::{
    delay_ms,
    prelude::*,
    simple_pwm::{IntoPwmPin, Timer0Pwm, Timer3Pwm},
};
use panic_halt as _;

// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let pins = arduino_hal::pins!(dp);
//     let mut duty_cycle = 0;
//     let timer = Timer3Pwm::new(dp.TC3, arduino_hal::simple_pwm::Prescaler::Direct);
//     let mut blue_led_pwm = pins.d2.into_output().into_pwm(&timer);
//     let mut is_brightening = true;
//     blue_led_pwm.set_duty(duty_cycle);
//     blue_led_pwm.enable();
//
//     let timer0 = Timer0Pwm::new(dp.TC0, arduino_hal::simple_pwm::Prescaler::Direct);
//     let mut buzzer_pwm = pins.d4.into_output().into_pwm(&timer0);
//     buzzer_pwm.set_duty(duty_cycle);
//     buzzer_pwm.enable();
//
//     loop {
//         delay_ms(1);
//         if is_brightening {
//             blue_led_pwm.set_duty(duty_cycle);
//             buzzer_pwm.set_duty(255_u8.saturating_sub(duty_cycle));
//             duty_cycle = duty_cycle.saturating_add(1);
//             if duty_cycle == u8::MAX {
//                 is_brightening = !is_brightening;
//             }
//         } else {
//             blue_led_pwm.set_duty(duty_cycle);
//             buzzer_pwm.set_duty(255_u8.saturating_sub(duty_cycle));
//             duty_cycle = duty_cycle.saturating_sub(1);
//             if duty_cycle == u8::MIN {
//                 is_brightening = !is_brightening;
//             }
//         }
//     }
// }

// const DEFAULT_BAUD_RATE: u32 = 57600;
//
// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let pins = arduino_hal::pins!(dp);
//     let mut serial = arduino_hal::default_serial!(dp, pins, DEFAULT_BAUD_RATE);
//     loop {
//         if let Ok(b) = nb::block!(serial.read()) {
//             ufmt::uwriteln!(&mut serial, "Hello, World!").unwrap_infallible();
//         }
//     }
// }

const DEFAULT_BAUD_RATE: u32 = 57600;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, DEFAULT_BAUD_RATE);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let temp = pins.a0.into_analog_input(&mut adc);
    loop {
        let reading = temp.analog_read(&mut adc);
        let celsius = (reading as f32 * (5000.0 / 1024.0)) / 10.0; // 10mV is LM35 constant/per degree C.
        let displayed_temp = celsius as u32;
        // ufmt::uwriteln!(&mut serial, "{}mv", reading).unwrap_infallible();
        ufmt::uwriteln!(&mut serial, "{}C", celsius as u32).unwrap_infallible();
    }
}
