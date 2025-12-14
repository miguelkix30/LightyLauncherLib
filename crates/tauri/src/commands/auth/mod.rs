mod types;
mod offline;
mod microsoft;
mod azuriom;

pub use types::{AuthConfig, AuthResult};
pub use offline::authenticate_offline;
pub use microsoft::authenticate_microsoft;
pub use azuriom::authenticate_azuriom;
