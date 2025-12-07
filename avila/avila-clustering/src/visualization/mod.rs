//! Módulo de visualização - exportação de dados para visualização
//!
//! Gera dados estruturados para integração com bibliotecas de visualização
//! como D3.js, Plotly, etc. Não realiza renderização gráfica direta.

pub mod projection;
pub mod dendrogram;
pub mod graph_layout;
pub mod export;

pub use projection::*;
pub use dendrogram::*;
pub use graph_layout::*;
pub use export::*;
