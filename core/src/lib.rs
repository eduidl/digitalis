#![warn(missing_debug_implementations, clippy::all, clippy::nursery)]

pub mod client;
pub mod common;
pub mod error;
pub mod schema;
pub mod server;
pub use error::{DigitalisError, DigitalisResult};

pub trait MessageMinimal: Send + 'static {
    fn write_to_bytes(&self) -> Result<Vec<u8>, protobuf::Error>;
}

impl<T> MessageMinimal for T
where
    T: protobuf::Message,
{
    fn write_to_bytes(&self) -> Result<Vec<u8>, protobuf::Error> {
        self.write_to_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Control {
    Exit,
    Continue,
}
