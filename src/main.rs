#![no_std]
#![no_main]

mod servo;
use arduino_hal::delay_ms;
use servo::Servo;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let pot = pins.a0.into_analog_input(&mut adc);

    let servo = Servo::new(pins.d9, dp.TC1);

    loop {
        let pot_value = pot.analog_read(&mut adc);
        let angle = (pot_value * 35) / 200; // 180 / 1024 * 100 = ~17.5
        let duty = (pot_value / 2) + 100;

        ufmt::uwriteln!(serial, "pot_value: {}, angle: {}, duty: {}", pot_value, angle, duty);
        
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