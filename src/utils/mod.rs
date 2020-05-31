pub mod alpaka_deserializers;
pub mod alpaka_serializers;

pub mod alpaka_error;
pub use alpaka_error::*;

pub mod alpaka_mode;
pub use alpaka_mode::*;

pub type NoBody = Option<i32>;
