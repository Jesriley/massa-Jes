// Copyright (c) 2022 MASSA LABS <info@massa.net>

//! this file defines all possible execution error categories

use displaydoc::Display;
use thiserror::Error;

/// Errors of the execution component.
#[non_exhaustive]
#[derive(Clone, Display, Error, Debug)]
pub enum ExecutionError {
    /// Channel error
    ChannelError(String),

    /// Runtime error: {0}
    RuntimeError(String),

    /// `MassaHashError`: {0}
    MassaHashError(#[from] massa_hash::MassaHashError),

    /// `ModelsError`: {0}
    ModelsError(#[from] massa_models::ModelsError),

    /// RollBuy error: {0}
    RollBuyError(String),

    /// RollSell error: {0}
    RollSellError(String),

    /// Block gas error: {0}
    BlockGasError(String),

    /// Inlcude operation error: {0}
    InlcudeOperationError(String),
}
