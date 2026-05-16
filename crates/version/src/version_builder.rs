use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use lighty_core::AppState;
use lighty_loaders::mods::request::ModRequest;
use lighty_loaders::types::VersionInfo;

/// Configures a Minecraft instance: name, loader, versions, and on-disk paths.
///
/// Default directories are derived from the global [`AppState`]:
/// - `game_dirs`   = `AppState::data_dir().join(name)`
/// - `runtime_dir` = alias of `game_dirs` until overridden
/// - `java_dirs`   = `AppState::config_dir().join("jre")`
///
/// Call [`AppState::init`] once at startup before constructing any
/// `VersionBuilder`.
///
/// # Example
/// ```rust
/// use lighty_core::AppState;
/// use lighty_loaders::types::Loader;
/// use lighty_version::VersionBuilder;
///
/// AppState::init("LightyLauncher")?;
///
/// let builder = VersionBuilder::new("my-profile", Loader::Vanilla, "", "1.21.1");
///
/// // Relocate the JVM-runtime folder (mods/saves/options.txt):
/// //   builder.launch(...).with_arguments()
/// //       .set(KEY_GAME_DIRECTORY, "runtime").done()    // → {data_dir}/{name}/runtime
/// //       .set(KEY_GAME_DIRECTORY, "/mnt/games").done() // → /mnt/games (absolute)
/// ```
#[derive(Debug, Clone)]
pub struct VersionBuilder<L = ()> {
    pub name: String,
    pub loader: L,
    pub loader_version: String,
    pub minecraft_version: String,
    pub game_dirs: PathBuf,
    pub java_dirs: PathBuf,
    /// Effective runtime dir. Initialised to `game_dirs` at `new()`
    /// — the launcher doesn't impose a `/runtime` subfolder. The
    /// runner overwrites this via [`VersionInfo::set_runtime_dir`]
    /// when the user supplied `arg_overrides[KEY_GAME_DIRECTORY]`
    /// through `LaunchBuilder::with_arguments()`. Not part of the
    /// user-facing builder API.
    pub runtime_dir: PathBuf,
    /// Mods the user attached via [`Self::with_mod`].
    ///
    /// Populated by [`ModSourcesBuilder`] before any HTTP call —
    /// actual resolution happens at install time inside the launch
    /// crate.
    pub mod_requests: Vec<ModRequest>,
}

impl<L> VersionBuilder<L> {
    /// Creates a new `VersionBuilder` with default paths derived
    /// from the global [`AppState`].
    ///
    /// Panics if [`AppState::init`] hasn't been called yet — that's
    /// a programmer error, not a runtime condition.
    pub fn new(
        name: &str,
        loader: L,
        loader_version: &str,
        minecraft_version: &str,
    ) -> Self {
        let game_dirs = AppState::data_dir().join(name);
        let java_dirs = AppState::config_dir().join("jre");
        Self {
            name: name.to_string(),
            loader,
            loader_version: loader_version.to_string(),
            minecraft_version: minecraft_version.to_string(),
            runtime_dir: game_dirs.clone(),
            game_dirs,
            java_dirs,
            mod_requests: Vec::new(),
        }
    }

    /// Opens the mod-sources sub-builder.
    ///
    /// Chain `.with_modrinth(...)` / `.with_curseforge(...)` on the
    /// returned [`ModSourcesBuilder`] and finalise with `.done()` —
    /// the accumulated [`ModRequest`]s are appended to
    /// [`Self::mod_requests`] and the install pipeline resolves them
    /// from the remote sources before launch.
    ///
    /// # Example
    /// ```rust
    /// instance
    ///     .with_mod()
    ///         .with_modrinth(vec![("sodium", None)])
    ///         .done()
    ///     .launch(&profile, JavaDistribution::Temurin)
    ///     .run().await?;
    /// ```
    pub fn with_mod(self) -> ModSourcesBuilder<L> {
        ModSourcesBuilder {
            parent: self,
            pending: Vec::new(),
        }
    }

    /// Overrides the Java install directory.
    pub fn with_custom_java_dir(mut self, java_dir: PathBuf) -> Self {
        self.java_dirs = java_dir;
        self
    }

    /// Replaces the loader.
    pub fn with_loader(mut self, loader: L) -> Self {
        self.loader = loader;
        self
    }

    /// Replaces the loader version.
    pub fn with_loader_version(mut self, version: &str) -> Self {
        self.loader_version = version.to_string();
        self
    }

    /// Replaces the Minecraft version.
    pub fn with_minecraft_version(mut self, version: &str) -> Self {
        self.minecraft_version = version.to_string();
        self
    }
}

impl<L: Clone + Send + Sync + Debug> VersionInfo for VersionBuilder<L> {
    type LoaderType = L;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.loader_version
    }

    fn minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    fn game_dirs(&self) -> &Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        &self.loader
    }

    fn mod_requests(&self) -> &[ModRequest] {
        &self.mod_requests
    }

    fn runtime_dir(&self) -> &Path {
        &self.runtime_dir
    }

    fn set_runtime_dir(&mut self, path: PathBuf) {
        self.runtime_dir = path;
    }
}

// Impl for &VersionBuilder so callers can pass borrowed builders.
// Read-only — the no-op `set_runtime_dir` default from the trait
// applies (can't mutate through a shared reference).
impl<'b, L: Clone + Send + Sync + Debug> VersionInfo for &'b VersionBuilder<L> {
    type LoaderType = L;

    fn name(&self) -> &str {
        &self.name
    }

    fn loader_version(&self) -> &str {
        &self.loader_version
    }

    fn minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    fn game_dirs(&self) -> &Path {
        &self.game_dirs
    }

    fn java_dirs(&self) -> &Path {
        &self.java_dirs
    }

    fn loader(&self) -> &Self::LoaderType {
        &self.loader
    }

    fn mod_requests(&self) -> &[ModRequest] {
        &self.mod_requests
    }

    fn runtime_dir(&self) -> &Path {
        &self.runtime_dir
    }
}

/// Sub-builder accumulating [`ModRequest`]s from one or more sources.
///
/// Mirrors the `LaunchBuilder` sub-builder pattern: hold the parent,
/// collect mutations locally, thread them back through `.done()`.
///
/// Each `.with_<source>(list)` call is synchronous — no HTTP work
/// happens here. The launch crate's resolver does the actual fetch
/// during `install()`, using the same JRE and event bus as the rest
/// of the pipeline.
pub struct ModSourcesBuilder<L> {
    parent: VersionBuilder<L>,
    pending: Vec<ModRequest>,
}

impl<L> ModSourcesBuilder<L> {
    /// Adds Modrinth mod requests.
    ///
    /// Each tuple is `(project-slug-or-id, optional-mod-version-id)`.
    /// `version` is the Modrinth release id, **not** the Minecraft
    /// version — MC + loader come from the instance and are used by
    /// the resolver to pick the latest compatible release when no
    /// explicit version was pinned.
    #[cfg(feature = "modrinth")]
    pub fn with_modrinth<S>(mut self, list: Vec<(S, Option<String>)>) -> Self
    where
        S: Into<String>,
    {
        for (id_or_slug, version) in list {
            self.pending.push(ModRequest::Modrinth {
                id_or_slug: id_or_slug.into(),
                version,
            });
        }
        self
    }

    /// Adds CurseForge mod requests.
    ///
    /// Each tuple is `(numeric-mod-id, optional-numeric-file-id)`.
    /// Requires [`lighty_loaders::mods::curseforge::set_api_key`] to
    /// have been called before `.run()`.
    #[cfg(feature = "curseforge")]
    pub fn with_curseforge(mut self, list: Vec<(u32, Option<u32>)>) -> Self {
        for (mod_id, file_id) in list {
            self.pending.push(ModRequest::CurseForge { mod_id, file_id });
        }
        self
    }

    /// Threads the accumulated requests back into the parent builder.
    pub fn done(mut self) -> VersionBuilder<L> {
        self.parent.mod_requests.append(&mut self.pending);
        self.parent
    }
}
