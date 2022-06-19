//! lines module includes all the lines as types, that appear in an SDP message.

pub mod common;

pub mod active;
pub mod attribute;
pub mod bandwidth;
pub mod connection;
pub mod email;
pub mod key;
pub mod media;
pub mod origin;
pub mod phone;
pub mod repeat;
pub mod session_information;
pub mod session_name;
pub mod uri;
pub mod version;
pub mod zone;

#[doc(inline)]
pub use active::Active;
#[doc(inline)]
pub use attribute::Attribute;
#[doc(inline)]
pub use bandwidth::Bandwidth;
#[doc(inline)]
pub use connection::Connection;
#[doc(inline)]
pub use email::Email;
#[doc(inline)]
pub use key::Key;
#[doc(inline)]
pub use media::Media;
#[doc(inline)]
pub use origin::Origin;
#[doc(inline)]
pub use phone::Phone;
#[doc(inline)]
pub use repeat::Repeat;
#[doc(inline)]
pub use session_information::SessionInformation;
#[doc(inline)]
pub use session_name::SessionName;
#[doc(inline)]
pub use uri::Uri;
#[doc(inline)]
pub use version::Version;
#[doc(inline)]
pub use zone::Zone;
