//! Tb67s128ftg Driver
//!
//! Platform-agnostic driver API for the Tb67s128ftg stepper motor driver. Can be
//! used on any platform for which implementations of the required
//! [embedded-hal] traits are available.
//!
//! For the most part, users are not expected to use this API directly. Please
//! check out [`Stepper`](crate::Stepper) instead.
//!
//! [embedded-hal]: https://crates.io/crates/embedded-hal

use core::convert::Infallible;

use embedded_hal::digital::{OutputPin, PinState};
use fugit::NanosDuration;

use crate::{step_mode::StepMode128, TimeStorageFormat, traits::{
    EnableDirectionControl, EnableStepControl, EnableStepModeControl,
    SetDirection, SetStepMode, Step as StepTrait,
}};

/// The Tb67s128ftg driver API
///
/// Users are not expected to use this API directly, except to create an
/// instance using [`tb67s128ftg::new`]. Please check out
/// [`Stepper`](crate::Stepper) instead.
pub struct tb67s128ftg<Enable, Standby, Reset, Mode0, Mode1, Mode2, Step, Dir>
{
    enable: Enable,
    standby: Standby,
    reset: Reset,
    mode0: Mode0,
    mode1: Mode1,
    mode2: Mode2,
    step: Step,
    dir: Dir,
}

impl tb67s128ftg<(), (), (), (), (), (), (), ()> {
    /// Create a new instance of `Tb67s128ftg`
    pub fn new() -> Self {
        Self {
            enable: (),
            standby: (),
            reset: (),
            mode0: (),
            mode1: (),
            mode2: (),
            step: (),
            dir: (),
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
EnableStepModeControl<(Reset, Mode0, Mode1, Mode2)>
for tb67s128ftg<(), (), (), (), (), (), Step, Dir>
    where
        Reset: OutputPin<Error = OutputPinError>,
        Mode0: OutputPin<Error = OutputPinError>,
        Mode1: OutputPin<Error = OutputPinError>,
        Mode2: OutputPin<Error = OutputPinError>,
{
    type WithStepModeControl =
    tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_step_mode_control(
        self,
        (reset, mode0, mode1, mode2): (Reset, Mode0, Mode1, Mode2),
    ) -> Self::WithStepModeControl {
        tb67s128ftg {
            enable: self.enable,
            standby: self.standby,
            reset,
            mode0,
            mode1,
            mode2,
            step: self.step,
            dir: self.dir,
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> SetStepMode
for tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
    where
        Reset: OutputPin<Error = OutputPinError>,
        Mode0: OutputPin<Error = OutputPinError>,
        Mode1: OutputPin<Error = OutputPinError>,
        Mode2: OutputPin<Error = OutputPinError>,
{
    // 19. AC Electrical Specification (page 42)
    // https://toshiba.semicon-storage.com/info/TB67S128FTG_datasheet_en_20210907.pdf?did=61073&prodName=TB67S128FTG
    const SETUP_TIME: NanosDuration<TimeStorageFormat> = NanosDuration::<TimeStorageFormat>::from_ticks(650); // Not specified in the datasheet, same as DRV8825 for now.
    const HOLD_TIME: NanosDuration<TimeStorageFormat> = NanosDuration::<TimeStorageFormat>::from_ticks(650); // Not specified in the datasheet, same as DRV8825 for now.

    type Error = OutputPinError;
    type StepMode = StepMode128;

    fn apply_mode_config(
        &mut self,
        step_mode: Self::StepMode,
    ) -> Result<(), Self::Error> {
        // Reset the device's internal logic.
        self.reset.set_high()?;

        use PinState::*;
        use StepMode128::*;
        let (mode0, mode1, mode2) = match step_mode {
            Full => (Low, Low, Low),
            M2 => (High, Low, Low),
            M4 => (Low, High, Low),
            M8 => (High, High, Low),
            M16 => (Low, Low, High),
            M32 => (High, Low, High),
            M64 => (Low, High, High),
            M128 => (High, High, High),
        };

        // Set mode signals.
        self.mode0.set_state(mode0)?;
        self.mode1.set_state(mode1)?;
        self.mode2.set_state(mode2)?;

        Ok(())
    }

    fn enable_driver(&mut self) -> Result<(), Self::Error> {
        self.reset.set_low()
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
EnableDirectionControl<Dir>
for tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, ()>
    where
        Dir: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl =
    tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_direction_control(self, dir: Dir) -> Self::WithDirectionControl {
        tb67s128ftg {
            enable: self.enable,
            standby: self.standby,
            reset: self.reset,
            mode0: self.mode0,
            mode1: self.mode1,
            mode2: self.mode2,
            step: self.step,
            dir,
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> SetDirection
for tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
    where
        Dir: OutputPin<Error = OutputPinError>,
{
    // 19. AC Electrical Specification (page 42)
    // https://toshiba.semicon-storage.com/info/TB67S128FTG_datasheet_en_20210907.pdf?did=61073&prodName=TB67S128FTG
    const SETUP_TIME: NanosDuration<TimeStorageFormat> = NanosDuration::<TimeStorageFormat>::from_ticks(650); // Not specified in the datasheet, same as DRV8825 for now.

    type Dir = Dir;
    type Error = Infallible;

    fn dir(&mut self) -> Result<&mut Self::Dir, Self::Error> {
        Ok(&mut self.dir)
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
EnableStepControl<Step>
for tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, (), Dir>
    where
        Step: OutputPin<Error = OutputPinError>,
{
    type WithStepControl =
    tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_step_control(self, step: Step) -> Self::WithStepControl {
        tb67s128ftg {
            enable: self.enable,
            standby: self.standby,
            reset: self.reset,
            mode0: self.mode0,
            mode1: self.mode1,
            mode2: self.mode2,
            step,
            dir: self.dir,
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> StepTrait
for tb67s128ftg<(), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
    where
        Step: OutputPin<Error = OutputPinError>,
{
    // 19. AC Electrical Specification (page 42)
    // https://toshiba.semicon-storage.com/info/TB67S128FTG_datasheet_en_20210907.pdf?did=61073&prodName=TB67S128FTG
    const PULSE_LENGTH: NanosDuration<TimeStorageFormat> = NanosDuration::<TimeStorageFormat>::from_ticks(300);

    type Step = Step;
    type Error = Infallible;

    fn step(&mut self) -> Result<&mut Self::Step, Self::Error> {
        Ok(&mut self.step)
    }
}
