//! Suporte WebXR para VR/AR

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Estado da sessão XR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrMode {
    /// Nenhuma sessão ativa
    None,
    /// Modo VR (imersivo)
    VR,
    /// Modo AR (realidade aumentada)
    AR,
}

#[cfg(target_arch = "wasm32")]
pub struct WebXRSession {
    mode: XrMode,
}

#[cfg(target_arch = "wasm32")]
impl WebXRSession {
    /// Inicia sessão VR
    pub async fn start_vr() -> Result<Self, JsValue> {
        // TODO: Implementar WebXR VR
        Ok(Self { mode: XrMode::VR })
    }

    /// Inicia sessão AR
    pub async fn start_ar() -> Result<Self, JsValue> {
        // TODO: Implementar WebXR AR
        Ok(Self { mode: XrMode::AR })
    }

    /// Para sessão
    pub fn stop(&mut self) {
        self.mode = XrMode::None;
    }

    /// Retorna modo atual
    pub fn mode(&self) -> XrMode {
        self.mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_mode() {
        assert_eq!(XrMode::None, XrMode::None);
        assert_ne!(XrMode::VR, XrMode::AR);
    }
}
