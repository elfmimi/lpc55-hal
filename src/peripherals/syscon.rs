//! API for system configuration (SYSCON) - always on
//!
//! The entry point to this API is [`SYSCON`]. Please refer to [`SYSCON`]'s
//! documentation for additional information.
//!
//! This module mostly provides infrastructure required by other parts of the
//! HAL API. For this reason, only a small subset of SYSCON functionality is
//! currently implemented.
//!
//! The SYSCON peripheral is described in the user manual, chapter 4.

// use core::marker::PhantomData;

// use crate::raw::syscon::{
//     // pdruncfg, presetctrl, starterp1, sysahbclkctrl, PDRUNCFG, PRESETCTRL, STARTERP1, SYSAHBCLKCTRL,
//     // UARTCLKDIV, UARTFRGDIV, UARTFRGMULT,
// };

// use cortex_m_semihosting::dbg;

// use crate::raw;
// use crate::{
//     time::{
//         self,
//         clock,
//     },
//     typestates::init_state,
// };

crate::wrap_always_on_peripheral!(Syscon, SYSCON);

impl Syscon {
    // TODO: relocate
    pub fn rev_id(&self) -> u8 {
        self.raw.dieid.read().rev_id().bits()
    }
}

/// The main API for the SYSCON peripheral
impl Syscon {
    /// Enables the clock for a peripheral or other hardware component
    pub fn enable_clock<P: ClockControl>(&mut self, peripheral: &mut P) {
        peripheral.enable_clock(self);
    }

    /// Disable peripheral clock
    pub fn disable_clock<P: ClockControl>(&mut self, peripheral: &mut P) {
        peripheral.disable_clock(self);
    }

    /// Check if peripheral clock is enabled
    pub fn is_clock_enabled<P: ClockControl>(&self, peripheral: &P) -> bool {
        peripheral.is_clock_enabled(&self)
    }

    /// Reset a peripheral
    pub fn reset<P: ResetControl>(&mut self, peripheral: &mut P) {
        peripheral.assert_reset(self);
        peripheral.clear_reset(self);
    }

}

/// TODO: do this systematically
/// By default, fro_12m is enabled in MAINCLKSELA
impl Syscon {
    // pub fn get_main_clk(&self) -> u8 {
    //     self.raw.mainclksela.read().sel().bits()
    // }

    // pub fn get_num_wait_states(&self) -> u8 {
    //     self.raw.fmccr.read().flashtim().bits()
    // }

    // pub fn set_num_wait_states(&mut self, num_wait_states: u8) {
    //     self.raw.fmccr.modify(|_, w| unsafe { w.flashtim().bits(num_wait_states) } );
    // }

    // pub fn set_ahbclkdiv(&self, div: u8) {
    //     assert!(div >= 1);
    //     // dbg!(self.raw.ahbclkdiv.read().div().bits());
    //     self.raw.ahbclkdiv.modify(unsafe { |_, w| w.div().bits(div - 1) });
    //     // dbg!(self.raw.ahbclkdiv.read().div().bits());
    // }

    // pub(crate) fn fro1mhz_as_main_clk(&mut self) {
    //     self.raw.mainclksela.modify(|_, w| w.sel().enum_0x2());
    // }

    // pub(crate) fn fro12mz_as_main_clk(&mut self) {
    //     // TODO: change these names in the PAC to their UM names
    //     // e.g. enum_0x0 -> fro_12m etc.
    //     self.raw.mainclksela.modify(|_, w| w.sel().enum_0x0());
    // }

    // pub(crate) fn fro96mhz_as_main_clk(&mut self) {
    //     // 1. may have to anactrl_fro192m_ctrl_ena_96mhzclk

    //     // 2. set voltage for 96MHz frequency

    //     // 3. set flash access cycles
    //     // formula is min(8, floor(9e-7*freq))
    //     // /* see fsl_clock.c, CLOCK_SetFLASHAccessCyclesForFreq */
    //     // in this case it's 8
    //     let num_wait_states = 8;
    //     self.set_num_wait_states(num_wait_states);

    //     // TODO: change these names in the PAC to their UM names
    //     // e.g. enum_0x0 -> fro_12m etc.
    //     self.raw.mainclksela.modify(|_, w| w.sel().enum_0x3());
    //     self.raw.mainclkselb.modify(|_, w| w.sel().enum_0x0());
    // }

    // /// TODO: Check if fro_hf is actually 96Mhz??
    // /// UM ANACTRL.FRO192M_CTRL.ENA_96MHZCLK says the 96Mhz clock
    // /// is disabled by default
    // pub fn fro_hf_as_usbfs_clk(&mut self) {
    //     // 96 Mhz via changing main clock and sourcing that
    //     // self.fro_hf_as_main_clk();
    //     // self.raw.usb0clksel.modify(|_, w| w.sel().enum_0x0());

    //     // Divide by n = 2 to get 48 Mhz (i.e., write (n - 1))
    //     dbg!(self.raw.usb0clkdiv.read().div().bits());
    //     self.raw
    //         .usb0clkdiv
    //         .modify(unsafe { |_, w| w.div().bits(0) });
    //     dbg!(self.raw.usb0clkdiv.read().div().bits());
    //     // Wait until the clock is stable (fsl_clock.c doesn't do this)
    //     while self.raw.usb0clkdiv.read().reqflag().is_ongoing() {}
    //     dbg!(self.raw.usb0clkdiv.read().div().bits());

    //     // Directly pick fro_hf as usbfs clock
    //     self.raw.usb0clksel.modify(|_, w| w.sel().enum_0x3());
    // }

    // pub fn is_enabled_usb0_hostm(&self) -> bool {
    //     self.raw.ahbclkctrl2.read().usb0_hostm().is_enable()
    // }

    // pub fn enable_usb0_hostm(&mut self) {
    //     self.raw.ahbclkctrl2.modify(|_, w| w.usb0_hostm().enable());
    // }

    // pub fn is_enabled_usb0_hosts(&self) -> bool {
    //     self.raw.ahbclkctrl2.read().usb0_hosts().is_enable()
    // }

    // pub fn enable_usb0_hosts(&mut self) {
    //     self.raw.ahbclkctrl2.modify(|_, w| w.usb0_hosts().enable());
    // }
}

/// Internal trait for controlling peripheral clocks
///
/// This trait is an internal implementation detail and should neither be
/// implemented nor used outside of LPC82x HAL. Any changes to this trait won't
/// be considered breaking changes.
///
/// Compared to https://git.io/fjpf9 (in lpc-rs/lpc8xx-hal/lpc8xx-hal-common)
/// we use a less minimal API in order to hide the fact that there are three
/// different AHLBCKLCTRL?, which a HAL user shouldn't really need to know about.
pub trait ClockControl {
    /// Internal method to enable a peripheral clock
    fn enable_clock(&self, s: &mut Syscon);

    /// Internal method to disable a peripheral clock
    fn disable_clock(&self, s: &mut Syscon);

    /// Check if peripheral clock is enabled
    fn is_clock_enabled(&self, s: &Syscon) -> bool;
}

//
// Unwrapped implementation for easier understanding
//
// impl ClockControl for raw::UTICK {
//     fn enable_clock<'h>(&self, h: &'h mut Handle) -> &'h mut Handle {
//         h.ahbclkctrl1.modify(|_, w| w.utick0().enable());
//         h
//     }

//     fn disable_clock<'h>(&self, h: &'h mut Handle) -> &'h mut Handle {
//         h.ahbclkctrl1.modify(|_, w| w.utick0().disable());
//         h
//     }

//     fn is_clock_enabled(&self, h: &Handle) -> bool {
//         h.ahbclkctrl1.read().utick0().is_enable()
//     }
// }

macro_rules! impl_clock_control {
    ($clock_control:ty, $clock:ident, $register:ident) => {
        impl ClockControl for $clock_control {
            fn enable_clock(&self, s: &mut Syscon) {
                s.raw.$register.modify(|_, w| w.$clock().enable());
                while s.raw.$register.read().$clock().is_disable() {}
            }

            fn disable_clock(&self, s: &mut Syscon) {
                s.raw.$register.modify(|_, w| w.$clock().disable());
            }

            fn is_clock_enabled(&self, s: &Syscon) -> bool {
                s.raw.$register.read().$clock().is_enable()
            }
        }
    };
}

impl_clock_control!(raw::ADC0, adc, ahbclkctrl0);
impl_clock_control!(raw::FLASH, flash, ahbclkctrl0);
impl_clock_control!(raw::FLEXCOMM0, fc0, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM1, fc1, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM2, fc2, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM3, fc3, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM4, fc4, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM5, fc5, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM6, fc6, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM7, fc7, ahbclkctrl1);
impl_clock_control!(raw::FLEXCOMM8, hs_lspi, ahbclkctrl2);
impl_clock_control!(raw::IOCON, iocon, ahbclkctrl0);
impl_clock_control!(raw::GINT0, gint, ahbclkctrl0);
impl_clock_control!(raw::PINT, pint, ahbclkctrl0);

impl_clock_control!(raw::USB0, usb0_dev, ahbclkctrl1);
impl_clock_control!(raw::USBFSH, usb0_hosts, ahbclkctrl2);  // well what about usb0_hostm?
impl_clock_control!(raw::UTICK0, utick, ahbclkctrl1);

impl_clock_control!(raw::ANACTRL, analog_ctrl, ahbclkctrl2);
impl_clock_control!(raw::CASPER, casper, ahbclkctrl2);
// there is no GPIO_SEC. what to do? create a PhantomData one?
// impl_clock_control!(raw::GPIO_SEC, gpio_sec, ahbclkctrl2);
impl_clock_control!(raw::PUF, puf, ahbclkctrl2);
impl_clock_control!(raw::RNG, rng, ahbclkctrl2);

// GPIO needs a separate implementation
impl ClockControl for raw::GPIO {
    fn enable_clock(&self, s: &mut Syscon) {
        s.raw.ahbclkctrl0.modify(|_, w| w.gpio0().enable());
        s.raw.ahbclkctrl0.modify(|_, w| w.gpio1().enable());
    }

    fn disable_clock(&self, s: &mut Syscon) {
        s.raw.ahbclkctrl0.modify(|_, w| w.gpio0().disable());
        s.raw.ahbclkctrl0.modify(|_, w| w.gpio1().disable());
    }

    #[allow(clippy::nonminimal_bool)]
    fn is_clock_enabled(&self, s: &Syscon) -> bool {
        s.raw.ahbclkctrl0.read().gpio0().is_enable() && s.raw.ahbclkctrl0.read().gpio1().is_enable()
    }
}

pub trait ResetControl {
    /// Internal method to assert peripheral reset
    fn assert_reset(&self, syscon: &mut Syscon);

    /// Internal method to clear peripheral reset
    fn clear_reset(&self, syscon: &mut Syscon);
}

macro_rules! impl_reset_control {
    ($reset_control:ty, $field:ident, $register:ident) => {
        impl<'a> ResetControl for $reset_control {
            fn assert_reset(&self, syscon: &mut Syscon) {
                syscon.raw.$register.modify(|_, w| w.$field().asserted());
                while syscon.raw.$register.read().$field().is_released() {}
            }

            fn clear_reset(&self, syscon: &mut Syscon) {
                syscon.raw.$register.modify(|_, w| w.$field().released());
                while syscon.raw.$register.read().$field().is_asserted() {}
            }
        }
    };
    ($reset_control:ty, $field1:ident, $field2:ident, $register:ident) => {
        impl<'a> ResetControl for $reset_control {
            fn assert_reset(&self, syscon: &mut Syscon) {
                syscon.raw.$register.modify(|_, w| w.$field1().asserted());
                while syscon.raw.$register.read().$field1().is_released() {}
                syscon.raw.$register.modify(|_, w| w.$field2().asserted());
                while syscon.raw.$register.read().$field2().is_released() {}
            }

            fn clear_reset(&self, syscon: &mut Syscon) {
                syscon.raw.$register.modify(|_, w| w.$field1().released());
                while syscon.raw.$register.read().$field1().is_asserted() {}
                syscon.raw.$register.modify(|_, w| w.$field2().released());
                while syscon.raw.$register.read().$field2().is_asserted() {}
            }
        }
    };
}

// to be completed
impl_reset_control!(raw::CASPER, casper_rst, presetctrl2);
impl_reset_control!(raw::FLEXCOMM0, fc0_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM1, fc1_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM2, fc2_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM3, fc3_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM4, fc4_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM5, fc5_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM6, fc6_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM7, fc7_rst, presetctrl1);
impl_reset_control!(raw::FLEXCOMM8, hs_lspi_rst, presetctrl2);
impl_reset_control!(raw::USB0, usb0_dev_rst, presetctrl1);
impl_reset_control!(raw::UTICK0, utick_rst, presetctrl1);

impl_reset_control!(raw::USBFSH, usb0_hostm_rst, usb0_hosts_rst, presetctrl2);
