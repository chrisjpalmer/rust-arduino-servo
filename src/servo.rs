use crate::map::{map, U16};

pub struct Servo {
    tc1: avr_device::atmega328p::TC1,
}

impl Servo {
    pub fn new(
        d9:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
        tc1: avr_device::atmega328p::TC1, 
    ) -> Self {
        d9.into_output(); //pb1/d9/oc1a

        // setup TC1 so that it has a 50Hz signal.
        // oc1a is connected to the control wire of the servo
        // 50Hz is achieved by dividing the 16Mhz clock source
        // of the arduino by the 64 prescaler and then again by 5000 ( which is the TOP value set via icr1 )
        tc1.tccr1a.write(|w| w.wgm1().bits(0b10).com1a().match_clear().com1b().match_clear());
        tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
        tc1.icr1.write(|w| unsafe { w.bits(4999) });

        Servo{tc1}
    }

    pub fn write_angle(&self, angle:u16) -> u16 {
        let duty = map(U16(angle), U16(0), U16(180), U16(100), U16(612));
        self.tc1.ocr1a.write(|w| unsafe { w.bits(duty.as_value()) });
        duty.as_value()
    }
}



