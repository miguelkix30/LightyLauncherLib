use crate::types::version_metadata::Version;
use crate::types::VersionInfo;
use std::time::Duration;
use crate::utils::error::QueryError;
use async_trait::async_trait;
use std::hash::Hash;

pub type Result<T> = std::result::Result<T, QueryError>;

/// Generic loader manifest query interface.
///
/// Implementors describe a loader's manifest source and how to extract
/// each sub-query (libraries, main class, etc.) from the raw payload.
/// `ManifestRepository<F>` then handles caching and concurrency.
#[async_trait]
pub trait Query: Send + Sync {
    /// Sub-query discriminator (e.g. `VanillaQuery`, `FabricQuery`).
    type Query: Eq + Hash + Clone + Send + Sync + 'static;

    /// Extracted payload returned to callers — typically [`VersionMetaData`].
    type Data: Clone + Send + Sync + 'static;

    /// Raw manifest type returned by [`Self::fetch_full_data`]
    /// (typically a JSON-deserialized struct).
    type Raw: Send + Sync + 'static;


    /// Loader group name (`"vanilla"`, `"forge"`, `"custom"`, ...).
    fn name() -> &'static str;

    /// Fetches the raw manifest from its remote source.
    async fn fetch_full_data<V: VersionInfo>(version: &V) -> Result<Self::Raw>;

    /// Extracts a typed sub-query from the raw manifest.
    async fn extract<V: VersionInfo>(version: &V, query: &Self::Query, raw: &Self::Raw) -> Result<Self::Data>;

    /// Default TTL applied to cached entries.
    fn cache_ttl() -> Duration {
        Duration::from_secs(3600) // 1h by default
    }

    /// Per-query TTL override (defaults to [`Self::cache_ttl`]).
    fn cache_ttl_for_query(_query: &Self::Query) -> Duration {
        Self::cache_ttl()
    }

    /// Builds the full [`Version`] (all sub-queries merged) from the raw manifest.
    async fn version_builder<V: VersionInfo>(version: &V, full_data: &Self::Raw) -> Result<Version>;
}


/// Cache key combining instance name and sub-query discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryKey<Q> {
    pub version: String,
    pub query: Q,
}
