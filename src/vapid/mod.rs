//! Contains tooling for signing with VAPID.

pub mod builder;
mod key;
mod signer;

pub use self::builder::VapidSignatureBuilder;
use self::key::VapidKey;
pub use self::signer::Claims;
pub use self::signer::VapidSignature;
use self::signer::VapidSigner;
