use crate::{
    scratchpad::{ELEVEN, NINE, TEN, TWELVE},
    FAMILY_CODE,
};
use thiserror::Error;

// /// Result
// pub type Result<T, E = Ds18b20Error> = core::result::Result<T, E>;

/// Error
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AError<T> {
    #[error(transparent)]
    Pin(T),
    #[error(transparent)]
    Ds18b20(#[from] Error),
}

/// The ds18b20 error
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("the bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't")]
    BusNotHigh,
    #[error("there are no devices attached to the 1-Wire bus")]
    NoAttachedDevices,
    #[error("timeout expired")]
    Timeout,
    #[error(transparent)]
    Crc(#[from] CrcError),
    #[error("unexpected family code {{ family_code={family_code}, expected={FAMILY_CODE} }}")]
    FamilyCode { family_code: u8 },
    #[error("unexpected configuration register {{ configuration_register={configuration_register:b}, expected=[{NINE:b}, {TEN:b}, {ELEVEN:b}, {TWELVE:b}] }}")]
    ConfigurationRegister { configuration_register: u8 },
}

/// The CRC error
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("unexpected CRC {{ crc={crc}, expected=0 }}")]
pub struct CrcError {
    pub(crate) crc: u8,
}
