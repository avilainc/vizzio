//! Serialização de modelos

use std::io::{Read, Write};

pub struct ModelSerializer;

impl ModelSerializer {
    pub fn save_to_json<W: Write>(&self, writer: &mut W, model_data: &str) -> Result<(), String> {
        writer.write_all(model_data.as_bytes())
            .map_err(|e| format!("Erro ao escrever: {}", e))
    }

    pub fn load_from_json<R: Read>(&self, reader: &mut R) -> Result<String, String> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)
            .map_err(|e| format!("Erro ao ler: {}", e))?;
        Ok(buffer)
    }
}
