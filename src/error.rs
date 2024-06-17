use thiserror::Error;
use maybe_http_client::HttpClientError;

#[derive(Debug, Error)]
#[error("ClientError: {0}")]
pub enum Error {
    // #[error("NewznabError: {0}")]
    // NewznabError(#[from] NewznabError),
    //
    #[error("ModelError: {0}")]
    JsonParsingError(#[from] serde_json::Error),

    // // Note that this type is boxed because its size might be very large in
    // // comparison to the rest. For more information visit:
    // // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    #[error("http error: {0}")]
    // Http(u16, String),
    Http(#[from] HttpClientError),

    #[error("http error({0}): {1}")]
    HttpStatusCode(u16, String),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
}