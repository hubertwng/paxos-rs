#![feature(option_filter)]
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate capnp;
extern crate futures;
#[macro_use]
extern crate log;
extern crate rand;
extern crate tokio_core;

mod algo;
mod messenger;
mod multipaxos;
mod net;
mod register;
mod config;

/// An instance is a "round" of Paxos. Instances are chained to
/// form a sequence of values.
///
/// In some implementations, this is also called a "Slot"
pub type Instance = u64;

#[allow(dead_code)]
mod messages_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/messages_capnp.rs"));
}
