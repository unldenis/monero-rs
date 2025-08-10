// Rust Monero Library
// Written in 2019-2023 by
//   Monero Rust Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//

//! Utility functions to manipulate addresses, amounts, keys, or ringct data types.
//!
//! Shared functions needed in different part of the library or utility types for external
//! integrations.
//!

/// Public 'address' module
pub mod address;
/// Public 'amount' module
pub mod amount;
/// Public 'test_utils' module (Only include this module if the "fuzzing" feature is enabled)
#[cfg(feature = "fuzzing")]
pub mod fuzz_utils;
/// Public 'key' module
pub mod key;
/// Public 'ringct' module
pub mod ringct;

use super::network;
use crate::blockdata::transaction;

/// A general error code, other errors should implement conversions to/from this if appropriate.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Monero network error.
    Network(network::Error),
    /// Monero address error.
    Address(address::Error),
    /// Monero key error.
    Key(key::Error),
    /// Monero RingCt error.
    RingCt(ringct::Error),
    /// Monero transaction error.
    Transaction(transaction::Error),
    /// Monero amount parsing error.
    AmountParsing(amount::ParsingError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Network(e) => write!(f, "Network error: {}", e),
            Error::Address(e) => write!(f, "Address error: {}", e),
            Error::Key(e) => write!(f, "Key error: {}", e),
            Error::RingCt(e) => write!(f, "RingCt error: {}", e),
            Error::Transaction(e) => write!(f, "Transaction error: {}", e),
            Error::AmountParsing(e) => write!(f, "Amount parsing error: {}", e),
        }
    }
}

impl From<network::Error> for Error {
    fn from(err: network::Error) -> Self {
        Error::Network(err)
    }
}

impl From<address::Error> for Error {
    fn from(err: address::Error) -> Self {
        Error::Address(err)
    }
}

impl From<key::Error> for Error {
    fn from(err: key::Error) -> Self {
        Error::Key(err)
    }
}

impl From<ringct::Error> for Error {
    fn from(err: ringct::Error) -> Self {
        Error::RingCt(err)
    }
}

impl From<transaction::Error> for Error {
    fn from(err: transaction::Error) -> Self {
        Error::Transaction(err)
    }
}

impl From<amount::ParsingError> for Error {
    fn from(err: amount::ParsingError) -> Self {
        Error::AmountParsing(err)
    }
}
