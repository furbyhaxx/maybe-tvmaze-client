#[cfg(all(feature = "async", feature = "sync"))]
compile_error!(
    "`async` and `sync` features cannot both be enabled at \
    the same time, if you want to use `blocking` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!(
    "You have to enable at least one of the \
    `async` or `sync` features."
);

mod error;
pub use error::Error;

mod client;
pub mod models;

pub use client::*;