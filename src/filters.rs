/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Filters for processing packets.
//!
//! In most cases, we would like Quilkin to do some preprocessing of received
//! packets before sending them off to their destination. Because this stage is
//! entirely specific to the use case at hand and differs between Quilkin
//! deployments, we must have a say over what tweaks to perform - this is where
//! filters come in.
//!
//! ### Filters and Filter chain
//! A filter represents a step in the tweaking/decision-making process of how we
//! would like to process our packets. For example, at some step, we might
//! choose to append some metadata to every packet we receive before forwarding
//! it while at a later step, choose not to forward packets that don't meet
//! some criteria.
//!
//! Quilkin lets us specify any number of filters and connect them in a sequence
//! to form a packet processing pipeline similar to a [Unix pipeline] - we call
//! this pipeline a `Filter chain`. The combination of filters and filter chain
//! allows us to add new functionality to fit every scenario without changing
//! Quilkin's core.
//!
//! As an example, say we would like to perform the following steps in our
//! processing pipeline to the packets we receive.
//!
//! * Append a predetermined byte to the packet.
//! * Compress the packet.
//! * Do not forward (drop) the packet if its compressed length is over
//!   512 bytes.
//!
//! We would create a filter corresponding to each step either by leveraging any
//! existing filters that do what we want or we could write one
//! ourselves and connect them to form the following filter chain:
//!
//! ```bash
//! append | compress | drop
//! ```
//!
//! When Quilkin consults our filter chain, it feeds the received packet into
//! `append` and forwards the packet it receives (if any) from `drop` - i.e the
//! output of `append` becomes the `input` into `compress` and so on in
//! that order.
//!
//! There are a few things we note here:
//!
//! * Although we have in this example, a filter called `drop`, every filter in
//!   the filter chain has complete ownership over a packet - if any filter
//!   drops a packet then no more work needs to be done regarding that packet so
//!   the next filter in the pipeline never has any knowledge that the dropped
//!   packet ever existed.
//!
//! * The filter chain is consulted for every received packet, and its filters
//!   are traversed in reverse order for packets travelling in the opposite
//!   direction. A packet received downstream will be fed into `append` and the
//!   result from `drop` is forwarded upstream - a packet received upstream will
//!   be fed into `drop` and the result from `append` is forwarded downstream.
//!
//! * Exactly one filter chain is specified and used to process all packets that
//!   flow through Quilkin.
//!
//!
//! ### Configuration Examples ###
//!
//! ```rust
//! # // Wrap this example within an async main function since the
//! # // local_rate_limit filter spawns a task on initialization
//! # #[tokio::main]
//! # async fn main() {
//! # let yaml = "
//! version: v1alpha1
//! static:
//!   filters:
//!     - name: quilkin.extensions.filters.debug.v1alpha1.Debug
//!       config:
//!         id: debug-1
//!     - name: quilkin.extensions.filters.local_rate_limit.v1alpha1.LocalRateLimit
//!       config:
//!         max_packets: 10
//!         period: 500ms
//!   endpoints:
//!     - address: 127.0.0.1:7001
//! # ";
//! # let config = quilkin::config::Config::from_reader(yaml.as_bytes()).unwrap();
//! # assert_eq!(config.source.get_static_filters().unwrap().len(), 2);
//! # quilkin::Builder::from(std::sync::Arc::new(config)).validate().unwrap();
//! }
//! ```
//!
//! We specify our filter chain in the `.filters` section of the proxy's
//! configuration which has takes a sequence of [`Filter`] objects. Each
//! object describes all information necessary to create a single filter.
//!
//! The above example creates a filter chain comprising a [`debug`]
//! filter followed by a [`local_rate_limit`] filter - the effect is
//! that every packet will be logged and the proxy will not forward more than
//! 20 packets per second.
//!
//! > The sequence determines the filter chain order so its ordering matters -
//!   the chain starts with the filter corresponding the first filter config and
//!   ends with the filter corresponding the last filter config in the sequence.
//!
//! ### Filter Dynamic Metadata
//!
//! A filter within the filter chain can share data within another filter
//! further along in the filter chain by propagating the desired data alongside
//! the packet being processed.  This enables sharing dynamic information at
//! runtime, e.g information about the current packet that might be useful to
//! other filters that process that packet.
//!
//! At packet processing time each packet is associated with _filter dynamic
//! metadata_ (a set of key-value pairs). Each key is a unique string while
//! value is an arbitrary value.  When a filter processes a packet, it can
//! choose to consult the associated dynamic metadata for more information or
//! itself add/update or remove key-values from the set.
//!
//! As an example, the built-in [`capture_bytes`] filter is one such filter that
//! populates a packet's filter metadata.
//!
//! [`capture_bytes`] extracts information (a configurable byte sequence) from
//! each packet and appends it to the packet's dynamic metadata for other
//! filters to leverage.
//!
//! On the other hand, the built-in [`token_router`] filter selects what
//! endpoint to route a packet by consulting the packet's dynamic metadata for a
//! routing token.
//!
//! Consequently, we can build a filter chain with a [`capture_bytes`] filter
//! preceeding a [`token_router`] filter, both configured to write and read the
//! same key in the dynamic metadata entry. The effect would be that packets are
//! routed to upstream endpoints based on token information extracted from
//! their contents.
//!
//! Refer to the [`metadata`] module for more information on specific well-known
//! metadata properties.
//!

mod error;
mod factory;
mod read;
mod registry;
mod set;
mod write;

pub(crate) mod chain;
pub(crate) mod manager;

pub mod capture_bytes;
pub mod compress;
pub mod concatenate_bytes;
pub mod debug;
pub mod load_balancer;
pub mod local_rate_limit;
pub mod metadata;
pub mod token_router;

/// Prelude containing all types and traits required to implement [`Filter`] and
/// [`FilterFactory`].
pub mod prelude {
    pub use super::{
        ConvertProtoConfigError, CreateFilterArgs, DynFilterFactory, Error, Filter, FilterFactory,
        ReadContext, ReadResponse, WriteContext, WriteResponse,
    };
}

// Core Filter types
pub use self::{
    error::{ConvertProtoConfigError, Error},
    factory::{CreateFilterArgs, DynFilterFactory, FilterFactory},
    read::{ReadContext, ReadResponse},
    registry::FilterRegistry,
    set::{FilterMap, FilterSet},
    write::{WriteContext, WriteResponse},
};

pub(crate) use self::chain::FilterChain;

/// Trait for routing and manipulating packets.
///
/// An implementation of [`Filter`] provides a `read` and a `write` method. Both
/// methods are invoked by the proxy when it consults the filter chain - their
/// arguments contain information about the packet being processed.
/// - `read` is invoked when a packet is received on the local downstream port
///   and is to be sent to an upstream endpoint.
/// - `write` is invoked in the opposite direction when a packet is received
///   from an upstream endpoint and is to be sent to a downstream client.
///
/// **Metrics**
///
/// * `filter_read_duration_seconds` The duration it took for a `filter`'s
///   `read` implementation to execute.
///   * Labels
///     * `filter` The name of the filter being executed.
///
/// * `filter_write_duration_seconds` The duration it took for a `filter`'s
///   `write` implementation to execute.
///   * Labels
///     * `filter` The name of the filter being executed.
pub trait Filter: Send + Sync {
    /// [`Filter::read`] is invoked when the proxy receives data from a
    /// downstream connection on the listening port.
    ///
    /// This function should return a [`ReadResponse`] containing the array of
    /// endpoints that the packet should be sent to and the packet that should
    /// be sent (which may be manipulated) as well. If the packet should be
    /// rejected, return [`None`].  By default, the context passes
    /// through unchanged.
    fn read(&self, ctx: ReadContext) -> Option<ReadResponse> {
        Some(ctx.into())
    }

    /// [`Filter::write`] is invoked when the proxy is about to send data to a
    /// downstream connection via the listening port after receiving it via one
    /// of the upstream Endpoints.
    ///
    /// This function should return an [`WriteResponse`] containing the packet to
    /// be sent (which may be manipulated). If the packet should be rejected,
    /// return [`None`]. By default, the context passes through unchanged.
    fn write(&self, ctx: WriteContext) -> Option<WriteResponse> {
        Some(ctx.into())
    }
}
