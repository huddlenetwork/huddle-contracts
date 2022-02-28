#[cfg(not(target_arch = "wasm32"))]
pub mod mocks;

pub mod models_app_links;
pub mod models_blocks;
pub mod models_chain_links;
pub mod models_common;
pub mod models_dtag_requests;
pub mod models_profile;
pub mod models_query;
pub mod models_relationships;
pub mod msg_router;
pub mod msg_routes;
pub mod querier;
pub mod query_router;
