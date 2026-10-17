#![no_std]
#![no_main]

/// Prints a line on serial: `log!(serial, "x = {}", x);`
macro_rules! log {
      ($serial:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
          ufmt::uwrite!($serial, $fmt $(, $arg)*).unwrap();
          ufmt::uwrite!($serial, "\r\n").unwrap();
      }};
}

use motor_ctr::{led::Led, step_motor::StepMotor};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // serial only takes pins: d0 and d1
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = Led::new(pins.d13.into_output());

    let mut step_motor = StepMotor::new(
        pins.d31.into_output(),
        pins.d32.into_output(),
        pins.d35.into_output(),
        pins.d36.into_output(),
    );

    log!(serial, "starting...\r");
    led.signal_led();
    step_motor.release().expect("failed releasing motor");
    arduino_hal::delay_ms(3000);

    log!(serial, "ready");
    log!(serial, "position: {}\r", step_motor.position());

    loop {
        led.on();

        step_motor.move_steps(2048);
        log!(serial, "position C: {}\r", step_motor.position());
        step_motor.release().expect("failed releasing motor");
        arduino_hal::delay_ms(1000);

        // half turn counter-clockwise, position() == 1024
        step_motor.move_steps(-1024);
        step_motor.release().expect("failed releasing motor");
        log!(serial, "position CC: {}\r", step_motor.position());
        arduino_hal::delay_ms(1000);

        // back to start, position() == 0
        step_motor.move_steps(-1024);
        step_motor.release().expect("failed releasing motor");
        log!(serial, "position CC: {}\r", step_motor.position());
        arduino_hal::delay_ms(1000);

        led.toggle();
    }
}
