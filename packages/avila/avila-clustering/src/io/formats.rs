//! Importação de diferentes formatos

use ndarray::Array2;
use std::io::BufRead;

pub struct DataImporter;

impl DataImporter {
    pub fn from_csv<R: BufRead>(&self, reader: R) -> Result<Array2<f64>, String> {
        let mut data = Vec::new();
        let mut n_cols = 0;

        for (line_idx, line_result) in reader.lines().enumerate() {
            let line = line_result.map_err(|e| format!("Erro ao ler linha: {}", e))?;

            if line_idx == 0 {
                continue; // Skip header
            }

            let values: Result<Vec<f64>, _> = line
                .split(',')
                .map(|s| s.trim().parse::<f64>())
                .collect();

            let row = values.map_err(|e| format!("Erro ao parsear linha {}: {}", line_idx, e))?;

            if n_cols == 0 {
                n_cols = row.len();
            } else if row.len() != n_cols {
                return Err(format!("Linha {} tem {} colunas, esperado {}", line_idx, row.len(), n_cols));
            }

            data.extend(row);
        }

        if data.is_empty() || n_cols == 0 {
            return Err("Dados vazios".to_string());
        }

        let n_rows = data.len() / n_cols;
        Array2::from_shape_vec((n_rows, n_cols), data)
            .map_err(|e| format!("Erro ao criar array: {}", e))
    }
}
