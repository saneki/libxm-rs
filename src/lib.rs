//! # libxm-rs
//! A binding of [libxm](https://github.com/Artefact2/libxm/) for Rust.
//!
//! A small XM (FastTracker II Extended Module) player library.
//! Designed for easy integration in demos and such, and provides timing
//! functions for easy sync against specific instruments, samples or channels.
//!
//! # Example
//! ```no_run
//! use libxm::XMContext;
//! use std::fs::File;
//! use std::io::Read;
//!
//! // Read the contents of the module into `data`
//! let mut data = Vec::new();
//! File::open("song.xm").unwrap().read_to_end(&mut data).unwrap();
//!
//! let mut xm = XMContext::new(&data, 48000).unwrap();
//! xm.set_max_loop_count(1);
//!
//! let mut buffer = [0.0; 4096];
//! while xm.loop_count() == 0 {
//!     xm.generate_samples(&mut buffer);
//!     // The buffer is filled with stereo PCM data. Use it for whatever you need...
//! }
//! // The song has looped once.
//! ```
//!
//! # Example
//! ```no_run
//! use libxm::XMContext;
//!
//! fn audio_callback(xm: &mut XMContext, buffer: &mut [f32]) {
//!     xm.generate_samples(buffer);
//! }
//! ```

#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub mod context;
pub mod ffi;

pub use context::{
    PlayingSpeed,
    Position,
    XMContext,
    XMError,
};
