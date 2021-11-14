#![no_std]
#![no_main]
// #![feature(unsigned_abs)]

mod f32_display;
mod servo;
mod map;

use core::ops::{DivAssign, MulAssign};

use arduino_hal::delay_ms;
// use panic_halt as _;


// issues:
// https://github.com/rust-lang/rust/issues/82242
// https://github.com/rust-lang/rust/issues/77131
// https://users.rust-lang.org/t/problems-writing-a-float-to-serial-port/52814/7
// https://github.com/jsen-/rust-i32-as-f32-issue-repro/blob/master/src/main.rs
// https://github.com/Rahix/avr-hal/issues/124
// compare with this ... https://github.com/crclark96/avr-division-repro

use servo::Servo;
use map::map;
use ufmt_float::uFmt_f32;
use half::f16;
use f32_display::UDisplayF32;
// use fixed::{types::extra::U3, FixedU16};

//https://users.rust-lang.org/t/problems-writing-a-float-to-serial-port/52814
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let pot = pins.a0.into_analog_input(&mut adc);

    let servo = Servo::new(pins.d9, dp.TC1);

    loop {
        // let pot_value = pot.analog_read(&mut adc) as i32;
        // let mut fraction = (pot_value as f32);
        // ufmt::uwriteln!(serial, "store fraction");
        // fraction = fraction / 1024_f32;
        // ufmt::uwriteln!(serial, "divide fraction");
        // fraction = fraction * 180_f32;
        // ufmt::uwriteln!(serial, "multiply fraction");
        // let fraction = fraction as i32;
        // let fraction = f16::from(pot_value as u8);
        // let angle = (fraction * f16::from(180)).to_f32();

        let pot_value = pot.analog_read(&mut adc);
        let angle = (pot_value * 35) / 200; // 180 / 1024 * 100 = ~17.5
        let duty = (pot_value / 2) + 100;
        // let angle2 = FixedU16::<U3>::from_num(pot_value) / 1024 * 180;
        // let mut angle2 = f16::from_bits(pot_value);
        // angle2.div_assign(1024_u16);
        // angle2.mul_assign(180_u16);
        // let angle2 = angle2.to_f32();

        //""/*UDisplayF32{value: fraction, decimal_places: 2}*/ /*uFmt_f32::Two(fraction)*/

        ufmt::uwriteln!(serial, "pot_value: {}, angle: {}, duty: {}", pot_value, angle, duty);
        // let (fraction, angle) = map(pot_value, 0, 1023, 0, 180);
        servo.write_duty(duty);
        delay_ms(5);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r");
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        );
    }

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}