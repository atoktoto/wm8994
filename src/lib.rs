#![no_std]

//! A minimal i2s driver for WM8994 audio codec.
//!
//! ## Usage example
//! ```no_run
//! use wm8994::Wm8994;
//!
//! let mut delay = ctx.core.SYST.delay(ccdr.clocks);
//! let gpiod = ctx.device.GPIOD.split(ccdr.peripheral.GPIOD);
//! let scl = gpiod.pd12.into_alternate_open_drain();
//! let sda = gpiod.pd13.into_alternate_open_drain();
//!
//! let i2c = ctx.device.I2C4.i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C4, &ccdr.clocks);
//! let mut driver = Wm8994::new(wm8994::Config { address: 0x1a }, &i2c, &delay);
//!
//! // init_headphone() expects MCLK to be present
//! driver.init_headphone().unwrap();
//! ```
//!
//! ## Limitations
//! This driver configures wm8994 to play audio from I2S to headphone outputs.
//! The format is fixed to 48 Khz, 16-bit I2S.
//!
//! ## Probing for the i2c the address
//! ```no_run
//! use wm8994::Wm8994;
//!
//! for addr in 0..0b111110 {
//!     let mut driver = Wm8994::new(wm8994::Config { address: addr }, &i2c, &delay);
//!     if let Ok(wm8994::registers::FAMILY_ID) = driver.get_family() {
//!         info!("Found at {:#04x}", addr);
//!     }
//! }
//!
//! ```

pub mod registers;

use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{SevenBitAddress};
use embedded_hal::blocking::{delay, i2c};

use registers::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error<I2cError> {
    I2c(I2cError),
}

pub struct Wm8994<I2c, I2cError, Delay> {
    config: Config,
    i2c: I2c,
    delay: Delay,

    // Error types must be bound to the object
    _i2c_err: PhantomData<I2cError>,
}

pub struct Config {
    pub address: SevenBitAddress,
}

impl<I2c, I2cError, Delay> Wm8994<I2c, I2cError, Delay>
    where
        I2c: i2c::Read<Error = I2cError> + i2c::Write<Error = I2cError> + i2c::WriteRead<Error = I2cError>,
        Delay: delay::DelayMs<u32>,
{

    pub fn new(config: Config, i2c: I2c, delay: Delay) -> Self {
        Self {
            config, i2c, delay,
            _i2c_err: PhantomData,
        }
    }

    pub fn init_headphone(&mut self) -> Result<(), Error<I2cError>> {

        assert_eq!(self.get_family()?, FAMILY_ID, "Incorrect family ID");

        // wm8994 Errata
        self.write_reg(0x102, 0x0003)?;
        self.write_reg(0x817, 0x0000)?;
        self.write_reg(0x102, 0x0000)?;

        /* Enable VMID soft start (fast), Start-up Bias Current Enabled: 0x006C at reg 0x39 */
        /* Bias Enable */
        self.write_reg(ANTIPOP2, 0x006C)?;

        /* Enable bias generator, Enable VMID */
        self.write_reg(PWR_MANAGEMENT_1, 0x0013)?;

        self.delay.delay_ms(50);

        // Headphone out

        /* Disable DAC1 (Left), Disable DAC1 (Right),
        Enable DAC2 (Left), Enable DAC2 (Right)*/
        self.write_reg(PWR_MANAGEMENT_5, 0x0303)?;

        /* Enable the AIF1 Timeslot 0 (Left) to DAC 1 (Left) mixer path */
        self.write_reg(AIF1_DAC1_LMR, 0x0001)?;

        /* Enable the AIF1 Timeslot 0 (Right) to DAC 1 (Right) mixer path */
        self.write_reg(AIF1_DAC1_RMR, 0x0001)?;

        /* Disable the AIF1 Timeslot 1 (Left) to DAC 2 (Left) mixer path */
        self.write_reg(AIF1_DAC2_LMR, 0x0000)?;

        /* Disable the AIF1 Timeslot 1 (Right) to DAC 2 (Right) mixer path */
        self.write_reg(AIF1_DAC2_RMR, 0x0000)?;

        // Clocks

        /* AIF1 Sample Rate = 48 (KHz), ratio=256 */
        self.write_reg(AIF1_RATE, 0x0083)?;

        /* AIF1 Word Length = 16-bits, AIF1 Format = I2S (Default Register Value) */
        self.aif1_control1_wl(0)?;

        /* slave mode */
        self.write_reg(AIF1_MASTER_SLAVE, 0x0000)?;

        /* Enable the DSP processing clock for AIF1, Enable the core clock */
        self.write_reg(CLOCKING1, 0x000A)?;

        /* Enable AIF1 Clock, AIF1 Clock Source = MCLK1 pin */
        self.write_reg(AIF1_CLOCKING1, 0x0001)?;

        /* Select DAC1 (Left) to Left Headphone Output PGA (HPOUT1LVOL) path */
        self.write_reg(OUTPUT_MIXER_1, 0x0100)?;

        /* Select DAC1 (Right) to Right Headphone Output PGA (HPOUT1RVOL) path */
        self.write_reg(OUTPUT_MIXER_2, 0x0100)?;

        /* Enable/Start the write sequencer */
        self.write_reg(WRITE_SEQ_CTRL1, 0x8100)?;

        assert_ne!(0, self.read_reg(WRITE_SEQ_CTRL2)?, "Sequencer not running");
        self.delay.delay_ms(325); //325
        assert_eq!(0, self.read_reg(WRITE_SEQ_CTRL2)?, "Sequencer not finished");

        /* Soft un-Mute the AIF1 Timeslot 0 DAC1 path L&R */
        self.write_reg(AIF1_DAC1_FILTER1,  0x0000)?;

        /* Unmute DAC 1 (Left) */
        self.write_reg(DAC1_LEFT_VOL, 0x00C0)?;

        /* Unmute DAC 1 (Right) */
        self.write_reg(DAC1_RIGHT_VOL, 0x00C0)?;

        /* Unmute the AIF1 Timeslot 0 DAC path */
        self.write_reg(AIF1_DAC1_FILTER1, 0x0010)?;

        /* Unmute DAC 2 (Left) */
        self.write_reg(DAC2_LEFT_VOL, 0x00C0)?;

        /* Unmute DAC 2 (Right) */
        self.write_reg(DAC2_RIGHT_VOL, 0x00C0)?;

        /* Unmute the AIF1 Timeslot 1 DAC2 path */
        self.write_reg(AIF1_DAC2_FILTER1, 0x0010)?;

        // /* Volume Control */
        self.set_output_volume(u8::MAX)?;

        Ok(())
    }

    pub fn aif1_control1_wl(&mut self, value: u16) -> Result<(), Error<I2cError>> {
        let mut tmp = self.read_reg(AIF1_CONTROL1)?;
        tmp &= !AIF1_CONTROL1_WL_MASK;
        tmp |= value << AIF1_CONTROL1_WL_POSITION;

        self.write_reg(AIF1_CONTROL1, tmp)?;
        Ok(())
    }

    pub fn set_mute(&mut self, mute_on: bool) -> Result<(), Error<I2cError>> {
        match mute_on {
            true => {
                /* Soft Mute the AIF1 Timeslot 0 DAC1 path L&R */
                self.write_reg(AIF1_DAC1_FILTER1, 0x0200)?;

                /* Soft Mute the AIF1 Timeslot 1 DAC2 path L&R */
                self.write_reg(AIF1_DAC2_FILTER1, 0x0200)?;
            }
            false => {
                /* Unmute the AIF1 Timeslot 0 DAC1 path L&R */
                self.write_reg(AIF1_DAC1_FILTER1, 0x0010)?;

                /* Unmute the AIF1 Timeslot 1 DAC2 path L&R */
                self.write_reg(AIF1_DAC2_FILTER1, 0x0010)?;
            }
        }

        Ok(())
    }

    pub fn set_output_volume(&mut self, volume: u8) -> Result<(), Error<I2cError>> {
        match volume {
            0 => {
                self.set_mute(true)?;
            },
            0..=0x3e => {
                self.set_mute(false)?;
                let tmp = volume as u16 | 0x140;
                self.write_reg(LEFT_OUTPUT_VOL, tmp)?;
                self.write_reg(RIGHT_OUTPUT_VOL, tmp)?;
                self.write_reg(SPK_LEFT_VOL, tmp)?;
                self.write_reg(SPK_RIGHT_VOL, tmp)?;

            },
            0x3f..=u8::MAX => {
                self.set_mute(false)?;
                let tmp = 0x3Fu16 | 0x140;
                self.write_reg(LEFT_OUTPUT_VOL, tmp)?;
                self.write_reg(RIGHT_OUTPUT_VOL, tmp)?;
                self.write_reg(SPK_LEFT_VOL, tmp)?;
                self.write_reg(SPK_RIGHT_VOL, tmp)?;
            }
        }

        Ok(())
    }

    pub fn soft_reset(&mut self) -> Result<(), Error<I2cError>>  {
        self.write_reg(0x0, 0)
    }

    pub fn get_family(&mut self) -> Result<u16, Error<I2cError>>  {
        self.read_reg(0x0)
    }

    pub fn read_reg(&mut self, reg: u16) -> Result<u16, Error<I2cError>>  {
        let mut buf: [u8; 2] = [0; 2];
        self.i2c.write_read(self.config.address, &reg.to_be_bytes(), &mut buf).map_err(|err| Error::I2c(err))?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn write_reg(&mut self, reg: u16, val: u16) -> Result<(), Error<I2cError>> {
        let reg = reg.to_be_bytes();
        let val = val.to_be_bytes();

        let mut buf: [u8; 4] = [0;4];
        buf[0] = reg[0];
        buf[1] = reg[1];
        buf[2] = val[0];
        buf[3] = val[1];

        self.i2c.write(self.config.address, &buf).map_err(|err| Error::I2c(err))
    }
}