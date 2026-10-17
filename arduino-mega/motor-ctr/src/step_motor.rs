//! 28BYJ-48 Stepper Motor with ULN2003
//! The motor has:
//! Step size = 11.25 degress
//! Steps per rotation: 32 x 64:1 gear = 2048 steps
//! 10-15 RPM
//!
//! Delay between phases:
//!     delay (ms) = 60_000 ms / (Steps per Revolution * RPM)
//!     delay (ms) = 60_000 us / 2048 x 15
//!
//! The sequence of control signals for 4 control wires is as follows:
//!
//! | Step | C1 | C2 | C3 | C4 |
//! |------|----|----|----|----|
//! |  1   | 1  | 1  | 0  | 0  |
//! |  2   | 0  | 1  | 1  | 0  |
//! |  3   | 0  | 0  | 1  | 1  |
//! |  4   | 1  | 0  | 0  | 1  |
//!
//! # Step modes
//!
//! - Full-step mode = 4-entry table, 2048 steps per revolution.
//! - Half-step mode = 8-entry table, 4096 per revolution
//!
use embedded_hal::digital::OutputPin;
const FULL_STEP_PHASES: [u8; 4] = [0b0011, 0b0110, 0b1100, 0b1001];
#[expect(dead_code)]
const STEPS_PER_REVOLUTION: u32 = 2048;
#[expect(dead_code)]
const MOTOR_RPM: u32 = 15;
#[expect(dead_code)]
const PHASE_DELAY: u32 = (60_000u32).div_ceil(STEPS_PER_REVOLUTION * MOTOR_RPM);

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub struct StepMotor<P1, P2, P3, P4> {
    pin_1: P1,
    pin_2: P2,
    pin_3: P3,
    pin_4: P4,
    /// Current index in the phase table
    phase: u8,
    /// Steps from zero. Positive is clockwise
    position: i32,
}

// Error = P1::Error is to use the same error type for all pins
impl<P1, P2, P3, P4> StepMotor<P1, P2, P3, P4>
where
    P1: OutputPin,
    P2: OutputPin<Error = P1::Error>,
    P3: OutputPin<Error = P1::Error>,
    P4: OutputPin<Error = P1::Error>,
{
    /// Creates a new motor without changing state.
    ///
    /// To power off the motor use [`Self::release`].
    pub fn new(pin_1: P1, pin_2: P2, pin_3: P3, pin_4: P4) -> Self {
        Self {
            pin_1,
            pin_2,
            pin_3,
            pin_4,
            phase: 0,
            position: 0,
        }
    }

    ///  Turn off motor
    ///
    /// Set all pins low so the coils don't heat while idle.
    pub fn release(&mut self) -> Result<(), P1::Error> {
        self.set_phase_bits(0b0000)
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn move_steps(&mut self, steps: i32) -> Result<(), P1::Error> {
        let direction = if steps >= 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };

        //  steps.abs() would panic in debug for i32::MIN,
        //  because +2,147,483,648 doesn't fit in an i32. unsigned_abs()
        //  returns a u32, where it fits.
        for _ in 0..steps.unsigned_abs() {
            self.step(direction)?;
            arduino_hal::delay_ms(3);
        }

        Ok(())
    }

    /// move the motor one step in a given [`Direction`]
    pub fn step(&mut self, direction: Direction) -> Result<(), P1::Error> {
        self.phase = self.next_phase_index(direction);
        let phase_bits = FULL_STEP_PHASES[self.phase as usize];
        self.set_phase_bits(phase_bits)?;

        let delta = match direction {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        };

        self.update_position(delta);

        Ok(())
    }

    fn next_phase_index(&self, dir: Direction) -> u8 {
        let steps_len = FULL_STEP_PHASES.len() as u8;

        match dir {
            Direction::Clockwise => (self.phase + 1) % steps_len,
            Direction::CounterClockwise => (self.phase + steps_len - 1) % steps_len,
        }
    }

    fn update_position(&mut self, delta: i32) {
        self.position = self.position.wrapping_add(delta);
    }

    /// Set the motor phase by setting the output pins state
    ///
    /// Note: `bit 0 = pin_1 (IN1)`
    fn set_phase_bits(&mut self, bits: u8) -> Result<(), P1::Error> {
        self.pin_1.set_state((bits & 0b0001 != 0).into())?;
        self.pin_2.set_state((bits & 0b0010 != 0).into())?;
        self.pin_3.set_state((bits & 0b0100 != 0).into())?;
        self.pin_4.set_state((bits & 0b1000 != 0).into())?;
        Ok(())
    }
}
