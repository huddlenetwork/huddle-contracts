use cosmwasm_std::{Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub const POST_REACTIONS_KEY: &[u8] = b"post_reactions";
pub const TOKEN_DENOM_KEY: &[u8] = b"denom";

/// Get a writable reactions amount bucket
pub fn reactions_store(storage: &mut dyn Storage) -> Bucket<Uint128> {
    bucket(storage, POST_REACTIONS_KEY)
}

/// Get a read-only reactions amount bucket
pub fn reactions_read(storage: &dyn Storage) -> ReadonlyBucket<Uint128> {
    bucket_read(storage, POST_REACTIONS_KEY)
}

/// Get a writable denom singleton
pub fn denom_store(storage: &mut dyn Storage) -> Singleton<String> {
    singleton(storage, TOKEN_DENOM_KEY)
}

/// Get a read-only denom singleton
pub fn denom_read(storage: &dyn Storage) -> ReadonlySingleton<String> {
    singleton_read(storage, TOKEN_DENOM_KEY)
}