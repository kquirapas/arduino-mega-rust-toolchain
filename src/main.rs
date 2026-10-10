#![no_std]
#![no_main]

// use arduino_hal::{
//     delay_ms,
//     prelude::*,
//     simple_pwm::{IntoPwmPin, Timer0Pwm, Timer3Pwm},
//     Pins,
// };
// use panic_halt as _;

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

// const DEFAULT_BAUD_RATE: u32 = 57600;
//
// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let pins = arduino_hal::pins!(dp);
//     let mut serial = arduino_hal::default_serial!(dp, pins, DEFAULT_BAUD_RATE);
//     let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
//     let temp = pins.a0.into_analog_input(&mut adc);
//     loop {
//         let reading = temp.analog_read(&mut adc);
//         let celsius = (reading as f32 * (5000.0 / 1024.0)) / 10.0; // 10mV is LM35 constant/per degree C.
//         let displayed_temp = celsius as u32;
//         // ufmt::uwriteln!(&mut serial, "{}mv", reading).unwrap_infallible();
//         ufmt::uwriteln!(&mut serial, "{}C", celsius as u32).unwrap_infallible();
//     }
// }

use arduino_hal::{
    delay_ms,
    i2c::Direction,
    port::{mode::Output, Pin},
    prelude::*,
    I2c, Pins,
};
use panic_halt as _;

const DEFAULT_BAUD_RATE: u32 = 57600;
const DELAY_TUNING: u32 = 1;
const CYCLES_TUNING: u32 = 1_000;
const I2C_SPEED: u32 = 50_000;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut d1 = pins.d8.into_output_high();
    let mut d2 = pins.d2.into_output_high();
    let mut d3 = pins.d3.into_output_high();
    let mut d4 = pins.d4.into_output_high();
    let mut a = pins.d31.into_output();
    let mut b = pins.d32.into_output();
    let mut c = pins.d33.into_output();
    let mut d = pins.d34.into_output();
    let mut e = pins.d35.into_output();
    let mut f = pins.d36.into_output();
    let mut g = pins.d37.into_output();
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(), // SDA
        pins.d21.into_pull_up_input(), // SCL
        I2C_SPEED,
    );
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let temp = pins.a0.into_analog_input(&mut adc);
    let mut serial = arduino_hal::default_serial!(dp, pins, DEFAULT_BAUD_RATE);

    let mut display = LedDisplay {
        digits: [
            d1.downgrade(),
            d2.downgrade(),
            d3.downgrade(),
            d4.downgrade(),
        ],
        segments: [
            a.downgrade(),
            b.downgrade(),
            c.downgrade(),
            d.downgrade(),
            e.downgrade(),
            f.downgrade(),
            g.downgrade(),
        ],
    };

    loop {
        let reading = temp.analog_read(&mut adc);
        let celsius = (reading as f32 * (5000.0 / 1024.0)) / 10.0; // 10mV is LM35 constant/per degree C.
        let displayed_temp = celsius as u32;
        ufmt::uwriteln!(&mut serial, "{}C", celsius as u32).unwrap_infallible();
        let value = (celsius as u8) % 100;
        let tens = value / 10;
        let ones = value % 10;

        set_cursor(&mut i2c, 0, 0);
        send_byte(&mut i2c, '0' as u8 + tens, true);
        send_byte(&mut i2c, 0x6, false);
        send_byte(&mut i2c, '0' as u8 + ones, true);
        send_byte(&mut i2c, 0x6, false);
        send_byte(&mut i2c, b' ', true);
        send_byte(&mut i2c, 0x6, false);
        send_byte(&mut i2c, b'C', true);
        send_byte(&mut i2c, 0x6, false);

        let mut cycles = 0;
        loop {
            display.reset_digits();
            display.reset_segments();

            display.toggle_digit(Digit::Four);
            display.display_celsius();

            delay_ms(DELAY_TUNING);

            display.reset_digits();
            display.reset_segments();

            display.toggle_digit(Digit::Two);
            display.display(ones);

            delay_ms(DELAY_TUNING);

            display.reset_digits();
            display.reset_segments();

            display.toggle_digit(Digit::One);
            display.display(tens);

            delay_ms(DELAY_TUNING);

            cycles += 1;
            if cycles >= CYCLES_TUNING {
                break;
            }
        }
    }
}

const SEGMENTS: [u8; 10] = [
    0b0111111, // 0: a b c d e f
    0b0000110, // 1: b c
    0b1011011, // 2: a b d e g
    0b1001111, // 3: a b c d g
    0b1100110, // 4: b c f g
    0b1101101, // 5: a c d f g
    0b1111101, // 6: a c d e f g
    0b0000111, // 7: a b c
    0b1111111, // 8: a b c d e f g
    0b1101111, // 9: a b c d f g
];

pub enum Digit {
    One = 0x0,
    Two,
    Three,
    Four,
}

pub struct LedDisplay {
    digits: [Pin<Output>; 4],
    segments: [Pin<Output>; 7],
}

impl LedDisplay {
    pub fn toggle_digit(&mut self, digit: Digit) {
        self.digits[digit as usize].set_low();
    }

    pub fn display(&mut self, number: u8) {
        let bits = SEGMENTS[number as usize];
        for i in 0..7 {
            let activated = (bits & (1 << i)) != 0;
            if activated {
                self.segments[i].set_high();
            }
        }
    }

    pub fn display_celsius(&mut self) {
        self.reset_segments();
        self.segments[0].set_high();
        self.segments[3].set_high();
        self.segments[4].set_high();
        self.segments[5].set_high();
    }

    pub fn reset_digits(&mut self) {
        for mut d in &mut self.digits {
            d.set_high();
        }
    }

    pub fn reset_segments(&mut self) {
        for mut s in &mut self.segments {
            s.set_low();
        }
    }
}

const LCD_ADDR: u8 = 0x27;
const BACKLIGHT: u8 = 0x08;
const ENABLE: u8 = 0x04;
const RS: u8 = 0x01;

fn expander_write(i2c: &mut I2c, value: u8) {
    i2c.write(LCD_ADDR, &[value]).unwrap();
}

fn send_nibble(i2c: &mut I2c, nibble: u8, is_data: bool) {
    let rs = if is_data { RS } else { 0 };
    let value = (nibble & 0xF0) | BACKLIGHT | rs;

    expander_write(i2c, value | ENABLE); // Enable high
    expander_write(i2c, value); // Enable low: latch nibble
}

fn send_byte(i2c: &mut I2c, byte: u8, is_data: bool) {
    send_nibble(i2c, byte & 0xF0, is_data); // upper nibble first
    send_nibble(i2c, byte << 4, is_data); // lower nibble
}

fn set_cursor(i2c: &mut I2c, row: u8, column: u8) {
    // Common offsets for 16x2 and 20x4 HD44780 displays
    let row_offset = match row {
        0 => 0x00,
        1 => 0x40,
        _ => return, // unsupported row
    };

    send_byte(i2c, 0x80 | (row_offset + column), false);
}
