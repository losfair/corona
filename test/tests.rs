extern crate corona;
extern crate futures;
extern crate tokio_core;
#[cfg(feature = "blocking-wrappers")]
extern crate tokio_io;
#[macro_use]
extern crate version_sync;

mod early_cleanup;
#[cfg(feature = "blocking-wrappers")]
mod io_blocking;
mod prelude_api;
mod recursive_core;
mod reentrant_wait;
mod version;
