//! Query processor (SQL-like)

use alloc::string::String;
use alloc::vec::Vec;

/// Parsed query
pub enum Query {
    /// SELECT
    Select {
        /// Colunas a serem retornadas
        columns: Vec<String>,
        /// Tabela alvo da consulta
        table: String,
        /// Claúsula WHERE opcional
        where_clause: Option<String>,
    },

    /// INSERT
    Insert {
        /// Tabela destino
        table: String,
        /// Colunas especificadas
        columns: Vec<String>,
        /// Valores correspondentes às colunas
        values: Vec<String>,
    },

    /// UPDATE
    Update {
        /// Tabela a atualizar
        table: String,
        /// Pares coluna/valor para atualização
        set: Vec<(String, String)>,
        /// Filtro opcional
        where_clause: Option<String>,
    },

    /// DELETE
    Delete {
        /// Tabela alvo
        table: String,
        /// Condição opcional
        where_clause: Option<String>,
    },

    /// CREATE TABLE
    CreateTable {
        /// Nome da tabela
        name: String,
        /// Definição das colunas
        columns: Vec<ColumnDef>,
    },
}

/// Definição de coluna
pub struct ColumnDef {
    /// Nome da coluna
    pub name: String,

    /// Tipo da coluna
    pub data_type: DataType,

    /// Constraints
    pub nullable: bool,
}

/// Tipos de dados
pub enum DataType {
    /// Integer
    Integer,
    /// Float
    Float,
    /// String (tamanho máximo)
    String(usize),
    /// Bytes
    Bytes,
}

/// Query executor
pub struct QueryExecutor {
    // TODO: Adicionar referência ao storage engine
}

impl QueryExecutor {
    /// Executa query
    pub fn execute(&mut self, query: Query) -> Result<QueryResult, String> {
        match query {
            Query::Select { columns, table, where_clause } => {
                self.execute_select(columns, table, where_clause)
            }
            Query::Insert { table, columns, values } => {
                self.execute_insert(table, columns, values)
            }
            _ => Err(String::from("Not implemented")),
        }
    }

    fn execute_select(
        &mut self,
        columns: Vec<String>,
        table: String,
        where_clause: Option<String>,
    ) -> Result<QueryResult, String> {
        let _ = (columns, table, where_clause);
        // TODO: Implementar SELECT
        Ok(QueryResult::empty())
    }

    fn execute_insert(
        &mut self,
        table: String,
        columns: Vec<String>,
        values: Vec<String>,
    ) -> Result<QueryResult, String> {
        let _ = (table, columns, values);
        // TODO: Implementar INSERT
        Ok(QueryResult::empty())
    }
}

/// Resultado da query
pub struct QueryResult {
    /// Linhas retornadas
    pub rows: Vec<Vec<String>>,
}

impl QueryResult {
    fn empty() -> Self {
        Self { rows: Vec::new() }
    }
}
