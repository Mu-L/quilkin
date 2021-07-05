/*
 * Copyright 2020 Google LLC All Rights Reserved.
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

//! Add a byte packet to either the beginning or end of each UDP packet.
//!
//! This is commonly used to provide an auth token to each packet, so they can
//! be routed appropriately.
//!
//! #### Filter name
//! ```text
//! quilkin.extensions.filters.concatenate_bytes.v1alpha1.ConcatenateBytes
//! ```
//!
//! ### Configuration Examples
//! ```rust
//! # let yaml = "
//! version: v1alpha1
//! static:
//!   filters:
//!     - name: quilkin.extensions.filters.concatenate_bytes.v1alpha1.ConcatenateBytes
//!       config:
//!           on_read: APPEND
//!           on_write: DO_NOTHING
//!           bytes: MXg3aWp5Ng==
//!   endpoints:
//!     - address: 127.0.0.1:7001
//! # ";
//! # let config = quilkin::config::Config::from_reader(yaml.as_bytes()).unwrap();
//! # assert_eq!(config.source.get_static_filters().unwrap().len(), 1);
//! # quilkin::proxy::Builder::from(std::sync::Arc::new(config)).validate().unwrap();
//! ```
//!
//! ### Metrics
//!
//! This filter currently exports no metrics.

mod config;

use crate::filters::prelude::*;

use config::ProtoConfig;
pub use config::{Config, Strategy};

pub const NAME: &str = "quilkin.extensions.filters.concatenate_bytes.v1alpha1.ConcatenateBytes";

/// Returns a factory for creating concatenation filters.
pub fn factory() -> DynFilterFactory {
    Box::from(ConcatBytesFactory)
}

/// The `ConcatenateBytes` filter's job is to add a byte packet to either the
/// beginning or end of each UDP packet that passes through. This is commonly
/// used to provide an auth token to each packet, so they can be
/// routed appropriately.
struct ConcatenateBytes {
    on_read: Strategy,
    on_write: Strategy,
    bytes: Vec<u8>,
}

impl ConcatenateBytes {
    pub fn new(config: Config) -> Self {
        ConcatenateBytes {
            on_read: config.on_read,
            on_write: config.on_write,
            bytes: config.bytes,
        }
    }
}

#[async_trait::async_trait]
impl Filter for ConcatenateBytes {
    async fn read(&self, mut ctx: ReadContext) -> Option<ReadResponse> {
        match self.on_read {
            Strategy::Append => {
                ctx.contents.extend(self.bytes.iter());
            }
            Strategy::Prepend => {
                ctx.contents.splice(..0, self.bytes.iter().cloned());
            }
            Strategy::DoNothing => {}
        }

        Some(ctx.into())
    }

    async fn write(&self, mut ctx: WriteContext<'async_trait>) -> Option<WriteResponse> {
        match self.on_write {
            Strategy::Append => {
                ctx.contents.extend(self.bytes.iter());
            }
            Strategy::Prepend => {
                ctx.contents.splice(..0, self.bytes.iter().cloned());
            }
            Strategy::DoNothing => {}
        }

        Some(ctx.into())
    }
}

#[derive(Default)]
struct ConcatBytesFactory;

impl FilterFactory for ConcatBytesFactory {
    fn name(&self) -> &'static str {
        NAME
    }

    fn create_filter(&self, args: CreateFilterArgs) -> Result<Box<dyn Filter>, Error> {
        Ok(Box::new(ConcatenateBytes::new(
            self.require_config(args.config)?
                .deserialize::<Config, ProtoConfig>(self.name())?,
        )))
    }
}
