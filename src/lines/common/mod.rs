//! Common types that might appear in more than one line types of an SDP message.

mod addrtype;
mod nettype;
mod typed_time;

pub use addrtype::Addrtype;
pub use nettype::Nettype;
pub use typed_time::TypedTime;
