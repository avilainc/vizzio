// Additional file formats module
pub mod macho;
pub mod dex;
pub mod wasm;
pub mod dotnet;
pub mod java;
pub mod python;

pub use macho::MachoParser;
pub use dex::DexParser;
pub use wasm::WasmParser;
