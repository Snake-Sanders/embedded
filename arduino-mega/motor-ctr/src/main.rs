#![no_std]
#![no_main]

mod step_motor;

use embedded_hal::digital::StatefulOutputPin;
use panic_halt as _;
use step_motor::{Direction, StepMotor};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // For examples (and inspiration), head to
    // https://github.com/Rahix/avr-hal/tree/main/examples
    let mut led = pins.d13.into_output();

    let mut step_motor = StepMotor::new(
        pins.d31.into_output(),
        pins.d32.into_output(),
        pins.d35.into_output(),
        pins.d36.into_output(),
    );

    loop {
        signal_led(&mut led);
        arduino_hal::delay_ms(1000);

        led.toggle();
        step_motor.step_n(300, Direction::Clockwise);
        led.toggle();
        arduino_hal::delay_ms(1000);

        led.toggle();
        step_motor.step_n(300, Direction::CounterClockwise);
        led.toggle();
        arduino_hal::delay_ms(1000);

        // led.toggle();
        // step_motor.rotate_degrees(-180);
        // led.toggle();
        // arduino_hal::delay_ms(1000);

        // led.toggle();
        // step_motor.rotate_degrees(90);
        // led.toggle();
        // step_motor.off();
        // arduino_hal::delay_ms(1000);
    }
}

// LED indicator with 3 bliks
fn signal_led<P: StatefulOutputPin>(led: &mut P) -> Result<(), P::Error> {
    for _ in 0..=6 {
        led.toggle()?;
        arduino_hal::delay_ms(200);
    }

    Ok(())
}
