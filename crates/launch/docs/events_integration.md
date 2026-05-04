# Eventos de instalación e integración de progreso

Este documento explica los eventos emitidos por el sistema de instalación/descarga y muestra ejemplos de integración en un launcher.

Resumen rápido:
- Hay dos tipos de progreso que puedes usar juntos:
  - Progreso por fase (conteo de archivos): `DownloadingAssets`, `DownloadingLibraries`, `DownloadingNatives`, `DownloadingMods` — útiles para mostrar "archivos completados / total".
  - Progreso en bytes (global): `InstallProgress { bytes }` — emitido por cada chunk descargado y por archivos completos; útil para barra de progreso en bytes/MB.

Dónde buscar los eventos en el código:
- Definición de eventos: [crates/event/src/module/launch.rs](crates/event/src/module/launch.rs#L1-L240)
- Emisión de progreso por fase y por bytes: [crates/launch/src/installer/downloader.rs](crates/launch/src/installer/downloader.rs#L1-L500)
- Uso en la instalación de assets: [crates/launch/src/installer/assets.rs](crates/launch/src/installer/assets.rs#L1-L200)

Eventos disponibles (resumen)
- `InstallStarted { version: String, total_bytes: u64 }`
  - Empleado para anunciar el inicio de la instalación. `total_bytes` puede usarse para calcular porcentaje combinado si dispones de todas las fases en bytes.
- `InstallProgress { bytes: u64 }`
  - Emitido por los descargadores (por chunk o archivo completo). Sumar `bytes` te da el progreso total en bytes desde el inicio.
- `DownloadingAssets { current: usize, total: usize }`
  - Emite conteo incremental por cada asset descargado (actualiza `current` hasta `total`). Ideal para mostrar "assets descargados / total assets".
- `DownloadingLibraries { current: usize, total: usize }`
  - Igual que arriba, para librerías.
- `DownloadingNatives { current: usize, total: usize }`
  - Igual, para archivos nativos.
- `DownloadingMods { current: usize, total: usize }`
  - Igual, para mods.
- `InstallCompleted { version: String, total_bytes: u64 }`
  - Se emite al finalizar la instalación.

Detalles de comportamiento
- Progreso por fase (conteo):
  - En `downloader::download_small_with_concurrency_limit` y `download_with_concurrency_limit` se crea un `ProgressState` con `total = tasks.len()` y un `AtomicUsize` que se incrementa cuando cada tarea finaliza; al incrementarse se emite el evento de fase correspondiente con `current` y `total`.
  - Esto significa que para assets (p.ej. ~2000 objetos) se emitirá `DownloadingAssets` N veces, incrementando `current` hasta `total` (no se emite por cada chunk dentro de un archivo, sino por cada archivo completado).

- Progreso en bytes (global):
  - En `download_small_file` y durante la lectura de `bytes_stream` en `download_large_file_once` se emite `InstallProgress { bytes }` por cada chunk o por el archivo completo cuando se lee todo en memoria.
  - Si quieres una barra en MB, acumula `bytes` desde `InstallStarted` hasta `InstallCompleted`.

Ejemplo de integración (pseudo-Rust) — suscribirse y calcular % por fase y por bytes

```rust
use lighty_event::{subscribe, Event};
use lighty_event::EventReceiver; // según re-export

// Asume que el launcher ya inicializó y tiene un `EventReceiver` o `subscribe()` disponible.
let mut rx = subscribe()?; // ejemplo conceptual

let mut total_bytes_expected: Option<u64> = None;
let mut accumulated_bytes: u64 = 0;

// Estado por fase
let mut assets_progress = (0usize, 0usize); // (current, total)
let mut libs_progress = (0usize, 0usize);

tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event {
            Event::Launch(launch) => match launch {
                LaunchEvent::InstallStarted { version, total_bytes } => {
                    total_bytes_expected = Some(total_bytes);
                    accumulated_bytes = 0;
                    // actualizar UI: iniciar barra
                }
                LaunchEvent::InstallProgress { bytes } => {
                    accumulated_bytes += bytes;
                    if let Some(total) = total_bytes_expected {
                        let pct = (accumulated_bytes as f64 / total as f64) * 100.0;
                        // actualizar UI: barra en % de bytes
                    } else {
                        // actualizar UI: mostrar MB descargados
                    }
                }
                LaunchEvent::DownloadingAssets { current, total } => {
                    assets_progress = (current, total);
                    // actualizar UI: "Assets: {}/{}" o barra de archivos
                }
                LaunchEvent::DownloadingLibraries { current, total } => {
                    libs_progress = (current, total);
                }
                LaunchEvent::InstallCompleted { version, total_bytes } => {
                    // instalación completa
                }
                _ => {}
            },
            _ => {}
        }
    }
});
```

Consejos de integración práctica
- Si quieres una sola barra de progreso "fase completa" (p.ej. assets), muestra `DownloadingAssets.current / total` y opcionalmente combina con `InstallProgress` para mostrar bytes dentro de la fase.
- Para una barra más precisa en porcentaje global, usa `InstallStarted.total_bytes` + acumula `InstallProgress.bytes`.
- Evita dividir el conteo de archivos por chunk; `DownloadingAssets` se cuenta por archivo completado. Si necesitas sub-archivo granular (p.ej. progreso de un archivo grande), escucha `InstallProgress` que viene por chunks.
- Ten en cuenta que las emisiones de fase se hacen cuando cada tarea termina — con concurrencia alta el UI recibirá saltos rápidos en `current` (varios incrementos cercanos entre sí).

Dónde adaptar en el código (si quieres cambiar comportamiento)
- `crates/launch/src/installer/downloader.rs`: `ProgressState::emit_progress()` y puntos donde se crea `ProgressState` en `download_*_with_concurrency_limit`.
- `crates/launch/src/installer/assets.rs`: pasa `Some(DownloadProgressKind::Assets)` al llamar al método de descarga para habilitar el evento de fase.

Preguntas frecuentes
- P: ¿Se emite algún evento por cada chunk además de `InstallProgress`?  
  R: No hay eventos de fase por chunk; solo `InstallProgress { bytes }` (chunks) y eventos de fase por archivo completado.

- P: ¿Cómo obtengo `total_bytes`?  
  R: `InstallStarted` contiene `total_bytes`. No todas las rutas de instalación pueden calcularlo perfectamente; si está disponible se emite al inicio.

Si quieres, puedo:
- añadir este archivo a la documentación (ya lo hice),
- actualizar `crates/launch/docs/events.md` con un resumen breve y link a este archivo,
- o implementar un ejemplo runnable que muestre un mock de descarga y cómo se verían los eventos en la UI. ¿Qué prefieres?
