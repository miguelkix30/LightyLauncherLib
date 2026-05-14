/// The supported Minecraft mod loaders.
///
/// Selects which manifest source and merge strategy the launcher uses
/// when fetching metadata for an instance.
#[derive(Debug, Clone)]
pub enum Loader {
    /// Fabric — modern, lightweight modding API.
    Fabric,
    /// NeoForge — community fork of Forge for MC 1.20.2+.
    NeoForge,
    /// OptiFine — graphics enhancement (not yet implemented).
    Optifine,
    /// Quilt — Fabric-compatible modding API.
    Quilt,
    /// Plain vanilla Minecraft (no loader).
    Vanilla,
    /// Forge — legacy modding API (not yet implemented).
    Forge,
    /// LightyUpdater — custom server-driven modpack delivery.
    LightyUpdater,
}
