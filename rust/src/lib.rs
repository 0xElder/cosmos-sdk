#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[doc(inline)]
pub use ixc_core::{Context, Response, EventBus, ensure, bail, fmt_error};
#[doc(inline)]
pub use ixc_core::resource::Resources;

#[doc(inline)]
pub use ixc_message_api::Address;
#[doc(inline)]
pub use ixc_schema::{StructCodec, EnumCodec, OneOfCodec};
#[doc(inline)]
pub use state_objects::*;
#[doc(inline)]
pub use simple_time::{Time, Duration};

#[cfg(feature = "core_macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate ixc_core_macros;
#[cfg(feature = "core_macros")]
#[doc(inline)]
pub use ixc_core_macros::*;

#[cfg(feature = "schema_macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate ixc_schema_macros;
#[cfg(feature = "schema_macros")]
#[doc(inline)]
pub use ixc_schema_macros::*;

#[cfg(feature = "state_objects_macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate state_objects_macros;
#[cfg(feature = "state_objects_macros")]
#[doc(inline)]
pub use state_objects_macros::*;
