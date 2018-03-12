//! This library implements helpers for managing various upstream
//! aortas.  It's used by the server as a central point to buffer up
//! changes that need to be flushed with the heartbeat and stores
//! config updates from upstream.
#![warn(missing_docs)]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate native_tls;
extern crate parking_lot;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate smith_aorta;
extern crate smith_common;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_timer;

mod auth;
mod heartbeat;
mod types;

pub use types::*;
