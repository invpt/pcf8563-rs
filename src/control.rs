//! Various functions related to the RTC control that are not specifically 
//! datetime-, timer-, alarm- or clock output-related will be defined here

use super::{PCF8563, DEVICE_ADDRESS, hal, Error, Register, BitFlags, Control, encode_bcd, decode_bcd};
use hal::blocking::i2c::{Write, WriteRead};

/// The two possible output types
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum InterruptOutput {    
    /// Active when TF active
    Continuous, 
    /// Pulsating according to the datasheet
    Pulsating,     
}

pub enum ClockControl {    
    /// Start the internal clock
    Start, 
    /// Start the internal clock
    Stop,     
}


impl<I2C, E> PCF8563<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    
    /// Enable or disable external clock test mode
    pub fn control_ext_clk_test_mode(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {
            /// Enable external clock test mode
            Control::Enable => {
                self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TEST1)       
            }
            /// Disable external clock test mode (normal operation)
            Control::Disable => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TEST1)
            }
        }
    }

    /*
    /// Enable external clock test mode
   pub fn enable_ext_clk_test_mode(&mut self) -> Result<(), Error<E>> {
       self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TEST1)
    }

    /// Disable external clock test mode (normal operation)
   pub fn disable_ext_clk_test_mode(&mut self) -> Result<(), Error<E>> {
       self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TEST1)
    }
    */

    /// Start/stop the clock 
    pub fn control_clock(&mut self, flag: ClockControl) -> Result<(), Error<E>> {
        match flag {
            ClockControl::Start => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::STOP)       
            }
            ClockControl::Stop => {
                self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::STOP)
            }
        }
    }

    /*
    /// RTC source clock runs
   pub fn start_clock(&mut self) -> Result<(), Error<E>> {
       self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::STOP)
    }

    /// RTC source clock stops
   pub fn stop_clock(&mut self) -> Result<(), Error<E>> {
       self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::STOP)
    }
    */

    /// Enable or disable power-on-reset override facility
     pub fn control_power_on_reset_override(&mut self, flag: Control) -> Result<(), Error<E>> {
        match flag {
            /// Enable power-on-reset override
            Control::Enable => {
                self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TESTC)       
            }
            /// Disable power-on-reset override (normal operation)
            Control::Disable => {
                self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TESTC)
            }
        }
    }

    /*
    /// Enable power-on-reset override facility
   pub fn enable_power_on_reset_override(&mut self) -> Result<(), Error<E>> {
       self.set_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TESTC)
    }

    /// Disable power-on-reset override (normal operation)
   pub fn disable_power_on_reset_override(&mut self) -> Result<(), Error<E>> {
       self.clear_register_bit_flag(Register::CTRL_STATUS_1, BitFlags::TESTC)
    }

    */

   /// Interrupt output when TF is active (continuous or pulsating)
   pub fn timer_interrupt_output(&mut self, output: InterruptOutput) -> Result<(), Error<E>> {
       match output {
           InterruptOutput::Continuous => {
               self.clear_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TI_TP)       
           }
           InterruptOutput::Pulsating => {
               self.set_register_bit_flag(Register::CTRL_STATUS_2, BitFlags::TI_TP)
           }
       }
   }


}


