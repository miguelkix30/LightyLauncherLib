//! Caching, querying, and error types shared by every loader implementation.
//!
//! [`query::Query`] is the trait each loader implements,
//! [`manifest::ManifestRepository`] is the cached generic repository that
//! wraps it, [`cache::Cache`] is the TTL-keyed async cache with
//! thundering-herd protection, and [`error::QueryError`] is the unified
//! error type returned by every loader operation.

pub mod manifest;
pub mod error;
pub mod cache;
pub mod query;