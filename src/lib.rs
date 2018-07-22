#![feature(futures_api)]

//! This library aims to provide a standard API for the interoperation of
//! other crates that implements (parts of) the Zigbee network, that
//! provides a protocol stack to be used in internet of things applications.
//! Its architecture follow the stack as proposed in the Zigbee
//! Specification document attached in the doc directory.
//! MAC and PHY layers are provided by the IEEE 802.15.4-2003 standard.
//! This library covers the upper layes as the Zigbee Standard does.
extern crate futures;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate bitfield;

use std::time::Duration;
use std::fmt;
use std::error::Error;
/// This module focus on the application layer.
pub mod apl;
pub mod nwk;
