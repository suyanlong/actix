//! # Actix is a rust actor framework.
//!
//! [Actors](https://actix.github.io/actix/actix/trait.Actor.html) are objects
//! which encapsulate state and behavior, they communicate exclusively
//! by exchanging messages. Actix actors are implemented on top of [Tokio](https://tokio.rs).
//! Mutiple actors could run in same thread. Actors could run in multiple threads
//! with suppoprt of [`Arbiter`](https://actix.github.io/actix/actix/struct.Arbiter.html).
//! Actors exchange typed messages.
//!
//! ## Features
//!
//! * Async/Sync actors.
//! * Actor communication in a local/thread context.
//! * Using Futures for asynchronous message handling.
//! * HTTP1/HTTP2 support ([actix-web](https://github.com/actix/actix-web))
//! * Actor supervision.
//! * Typed messages (No `Any` type). Generic messages are allowed.
//! * Minimum supported Rust version: 1.20 or later
#![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate libc;
extern crate uuid;
extern crate smallvec;
extern crate crossbeam;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_signal;

#[macro_use]
extern crate actix_derive;

#[cfg(test)]
extern crate bytes;

#[doc(hidden)]
pub use actix_derive::*;

mod actor;
mod arbiter;
mod address;
mod context;
mod contextimpl;
mod contextcells;
mod contextitems;
mod envelope;
mod framed;
mod handler;
pub mod queue;
mod message;
mod registry;
mod system;
mod supervisor;
mod utils;

pub mod fut;
pub mod actors;
pub mod msgs;
pub mod sync;

#[doc(hidden)]
pub mod constants;

pub use fut::{ActorFuture, ActorStream, WrapFuture, WrapStream, FinishStream};
pub use actor::{Actor, ActorState, FramedActor, Supervised,
                ActorContext, AsyncContext, SpawnHandle};
pub use handler::{Handler, ResponseType, MessageResult, ResponseFuture};
pub use arbiter::Arbiter;
pub use address::{Address, SyncAddress, Subscriber, ActorAddress};
pub use context::{Context, ContextFutureSpawner};
pub use envelope::ToEnvelope;
pub use framed::FramedContext;
pub use message::{Request, Response};
pub use sync::{SyncContext, SyncArbiter};
pub use registry::{Registry, SystemRegistry, ArbiterService, SystemService};
pub use system::{System, SystemRunner};
pub use utils::Condition;
pub use supervisor::Supervisor;

pub mod prelude {
//! The `actix` prelude
//!
//! The purpose of this module is to alleviate imports of many common actix traits
//! by adding a glob import to the top of actix heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use actix::prelude::*;
//! ```

#[doc(hidden)]
pub use actix_derive::*;

pub use fut::{ActorFuture, ActorStream, WrapFuture, WrapStream};
pub use actor::{Actor, ActorContext, AsyncContext, FramedActor};
pub use arbiter::Arbiter;
pub use address::{Address, SyncAddress};
pub use framed::FramedContext;
pub use message::Response;
pub use handler::{Handler, ResponseType, MessageResult, ResponseFuture};
pub use context::{Context, ContextFutureSpawner};
pub use system::System;
pub use sync::{SyncContext, SyncArbiter};

pub mod actix {
    pub use msgs;
    pub use fut::{self, ActorFuture, ActorStream, WrapFuture, WrapStream};
    pub use actor::{Actor, ActorState, FramedActor, Supervised,
                    ActorContext, AsyncContext, SpawnHandle};
    pub use handler::{Handler, ResponseType, MessageResult, ResponseFuture};
    pub use arbiter::Arbiter;
    pub use address::{Address, SyncAddress, Subscriber, ActorAddress};
    pub use context::{Context, ContextFutureSpawner};
    pub use framed::FramedContext;
    pub use message::{Request, Response};
    pub use system::System;
    pub use sync::{SyncContext, SyncArbiter};
    pub use registry::{ArbiterService, SystemService};
    pub use utils::Condition;
}}


pub mod dev {
//! The `actix` prelude for library developers
//!
//! The purpose of this module is to alleviate imports of many common actix traits
//! by adding a glob import to the top of actix heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use actix::dev::*;
//! ```

    pub use prelude::*;
    pub use prelude::actix::*;

    pub use queue;
    pub use address::{ActorAddress};
    pub use context::AsyncContextApi;
    pub use contextimpl::ContextImpl;
    pub use contextcells::{ContextProtocol, ContextCellResult,
                           ActorAddressCell, ActorItemsCell, ActorWaitCell};
    pub use envelope::{Envelope, ToEnvelope, RemoteEnvelope};
}
