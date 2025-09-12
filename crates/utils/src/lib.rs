pub mod marc;

pub mod prelude {
    pub use super::marc::MArc;
    pub use anyhow::{anyhow, bail, Context, Result};
    pub use proc_macros::main_pretty_error;
    pub use tracing::{
        debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
        warn_span, Span,
    };
}

pub mod crates {
    pub use anyhow;
    pub use proc_macros;
    pub use tracing;
    pub use tracing_subscriber;
}
