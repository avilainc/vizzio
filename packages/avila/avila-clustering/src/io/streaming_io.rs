//! Leitura de grandes arquivos em chunks

use std::io::BufRead;

pub struct StreamingReader {
    chunk_size: usize,
}

impl StreamingReader {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    pub fn read_chunks<R: BufRead, F>(&self, reader: R, mut callback: F) -> Result<(), String>
    where
        F: FnMut(Vec<Vec<f64>>) -> Result<(), String>,
    {
        let mut chunk = Vec::new();

        for (line_idx, line_result) in reader.lines().enumerate() {
            let line = line_result.map_err(|e| format!("Erro ao ler linha: {}", e))?;

            if line_idx == 0 {
                continue; // Skip header
            }

            let values: Result<Vec<f64>, _> = line
                .split(',')
                .map(|s| s.trim().parse::<f64>())
                .collect();

            chunk.push(values.map_err(|e| format!("Erro na linha {}: {}", line_idx, e))?);

            if chunk.len() >= self.chunk_size {
                callback(chunk.clone())?;
                chunk.clear();
            }
        }

        if !chunk.is_empty() {
            callback(chunk)?;
        }

        Ok(())
    }
}
