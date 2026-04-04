////////////////////////////////////////////////////////////////////////////////
/// @file       lib.rs
/// @author     Krisna Pranav
/// @date       April 3, 2026
/// @license    MIT License
/// @copyright  Copyright (c) 2026 Krisna Pranav
///
/// @brief      Main library entry point and public API for the hit core.
///
/// @details    This module serves as the primary entry point for the hit core
///             library, exposing the public API and re-exporting essential
///             components. It coordinates the various submodules including
///             error handling and core functionality. All public interfaces
///             are defined and documented here for external consumption.
///
////////////////////////////////////////////////////////////////////////////////

pub mod error;
pub mod hash;
mod objects;
mod dag;
