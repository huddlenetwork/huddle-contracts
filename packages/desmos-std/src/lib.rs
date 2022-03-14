#[cfg(not(target_arch = "wasm32"))]
pub mod mock;

pub mod msg;
pub mod profiles;
pub mod query;
pub mod relationships;
pub mod subspaces;
pub mod types;
