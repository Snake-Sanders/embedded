//! 28BYJ-48 Stepper Motor with ULN2003
//! The motor has:
//! Step size = 11.25 degress
//! Steps per rotation: 32 x 64:1 gear = 2048 steps
//! 10-15 RPM
//!
//! Delay between phases:
//!     delay (us) = 1 min / (Steps per Revolution) x RPM
//!     delay (us) = 60_000_000 us / 2048 x 15
//!
//! The sequence of control signals for 4 control wires is as follows:
//!
//! Step    C1 C2 C3 C4
//!    1    1  0  1  0
//!    2    0  1  1  0
//!    3    0  1  0  1
//!    4    1  0  0  1
//!
use embedded_hal::digital::OutputPin;

const FULL_STEP_PHASES: [u8; 4] = [0b1010, 0b0110, 0b0101, 0b1001];
const STEPS_PER_REVOLUTION: u32 = 2048;
const MOTOR_RPM: u32 = 15;
const PHASE_DELAY: u32 = (60_000u32).div_ceil(STEPS_PER_REVOLUTION * MOTOR_RPM);

#[derive(Clone, PartialEq, Copy)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub struct StepMotor<P1, P2, P3, P4> {
    pin_1: P1,
    pin_2: P2,
    pin_3: P3,
    pin_4: P4,
}

// Error = P1::Error is to use the same error type for all pins
impl<P1, P2, P3, P4> StepMotor<P1, P2, P3, P4>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    P3: OutputPin<Error = P1::Error>,
    P4: OutputPin<Error = P1::Error>,
{
    pub fn new(pin_1: P1, pin_2: P2, pin_3: P3, pin_4: P4) -> Self {
        Self {
            pin_1,
            pin_2,
            pin_3,
            pin_4,
        }
    }

    /// runs `n` number of steps in a given [Direction]
    pub fn step_n(&mut self, steps: usize, direction: Direction) -> Result<(), P1::Error> {
        (0..steps).for_each(|_i| {
            if self.do_full_step(direction).is_err() {
                //println!("step {i} failed");
                todo!();
            }
        });

        Ok(())
    }

    pub fn rotate_degrees(&mut self, degrees: isize) -> Result<(), P1::Error> {
        let steps: usize = ((degrees as usize * STEPS_PER_REVOLUTION as usize) / 360) as usize;
        let direction = direction_from_degrees(degrees);

        self.step_n(steps, direction)?;

        Ok(())
    }

    ///  Turn off motor
    ///
    /// Set all pins low so the coils don't heat while idle.
    pub fn off(&mut self) -> Result<(), P1::Error> {
        self.set_bits(0b0000)?;

        Ok(())
    }

    /// Performs on full step
    fn do_full_step(&mut self, direction: Direction) -> Result<(), P1::Error> {
        let mut step_phases = FULL_STEP_PHASES.clone();

        if direction == Direction::CounterClockwise {
            step_phases.reverse()
        };

        for bits in step_phases {
            self.set_bits(bits)?;
            arduino_hal::delay_ms(PHASE_DELAY);
        }

        Ok(())
    }

    /// Sets pins 1..4 according to the low nibble in [bits]
    fn set_bits(&mut self, bits: u8) -> Result<(), P1::Error> {
        self.pin_1.set_state((bits & 0b0001 != 0).into())?;
        self.pin_2.set_state((bits & 0b0010 != 0).into())?;
        self.pin_3.set_state((bits & 0b0100 != 0).into())?;
        self.pin_4.set_state((bits & 0b1000 != 0).into())?;
        Ok(())
    }
}

const fn direction_from_degrees(degrees: isize) -> Direction {
    if degrees > 0 {
        Direction::Clockwise
    } else {
        Direction::CounterClockwise
    }
}

#[cfg(test)]
mod tests {
    use supper::*;
    // enable std only for test
    // extern crate std;

    #[test]
    fn test_direction_from_degress() {
        assert_eq!(direction_from_degrees(0), Clockwise);
    }
}
