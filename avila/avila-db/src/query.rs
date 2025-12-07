//! Query engine (SQL-like)

use alloc::vec::Vec;
use alloc::string::String;

/// Query AST (Abstract Syntax Tree)
#[derive(Debug, Clone)]
pub enum Query {
    /// SELECT
    Select(SelectQuery),
    /// INSERT
    Insert(InsertQuery),
    /// UPDATE
    Update(UpdateQuery),
    /// DELETE
    Delete(DeleteQuery),
    /// CREATE TABLE
    CreateTable(CreateTableQuery),
}

/// SELECT query
#[derive(Debug, Clone)]
pub struct SelectQuery {
    /// Colunas a selecionar
    pub columns: Vec<String>,
    /// Tabela
    pub table: String,
    /// Condição WHERE
    pub where_clause: Option<Expr>,
    /// ORDER BY
    pub order_by: Vec<(String, OrderDirection)>,
    /// LIMIT
    pub limit: Option<usize>,
}

/// INSERT query
#[derive(Debug, Clone)]
pub struct InsertQuery {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<Value>>,
}

/// UPDATE query
#[derive(Debug, Clone)]
pub struct UpdateQuery {
    pub table: String,
    pub set: Vec<(String, Value)>,
    pub where_clause: Option<Expr>,
}

/// DELETE query
#[derive(Debug, Clone)]
pub struct DeleteQuery {
    pub table: String,
    pub where_clause: Option<Expr>,
}

/// CREATE TABLE query
#[derive(Debug, Clone)]
pub struct CreateTableQuery {
    pub table: String,
    pub columns: Vec<ColumnDef>,
}

/// Definição de coluna
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub primary_key: bool,
}

/// Tipos de dados
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Integer,
    BigInt,
    Float,
    Double,
    Text,
    Blob,
    Boolean,
    Timestamp,
}

/// Ordem de ordenação
#[derive(Debug, Clone, Copy)]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Expressão
#[derive(Debug, Clone)]
pub enum Expr {
    /// Literal value
    Literal(Value),
    /// Column reference
    Column(String),
    /// Binary operation
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    /// Function call
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

/// Operador binário
#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
}

/// Valor
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
    Boolean(bool),
}

/// Query executor
pub struct QueryExecutor;

impl QueryExecutor {
    /// Executa query
    pub fn execute(query: Query) -> Result<QueryResult, QueryError> {
        match query {
            Query::Select(select) => Self::execute_select(select),
            Query::Insert(insert) => Self::execute_insert(insert),
            Query::Update(update) => Self::execute_update(update),
            Query::Delete(delete) => Self::execute_delete(delete),
            Query::CreateTable(create) => Self::execute_create_table(create),
        }
    }

    fn execute_select(query: SelectQuery) -> Result<QueryResult, QueryError> {
        // Simplified execution:
        // 1. Acessar tabela (scan)
        // 2. Aplicar WHERE filter
        // 3. Projetar colunas selecionadas
        // 4. Ordenar (ORDER BY)
        // 5. Limitar (LIMIT)

        let mut rows = Vec::new();

        // Mock data para demonstração
        // Em produção: acessaria StorageEngine
        if query.table == "users" {
            rows.push(vec![
                Value::Integer(1),
                Value::Text(alloc::string::String::from("Alice")),
            ]);
            rows.push(vec![
                Value::Integer(2),
                Value::Text(alloc::string::String::from("Bob")),
            ]);
        }

        // Aplicar WHERE filter se existir
        if let Some(where_expr) = &query.where_clause {
            rows.retain(|row| Self::eval_expr(where_expr, row));
        }

        // Aplicar LIMIT
        if let Some(limit) = query.limit {
            rows.truncate(limit);
        }

        Ok(QueryResult::Rows(rows))
    }

    fn eval_expr(_expr: &Expr, _row: &Vec<Value>) -> bool {
        // Simplified: sempre true
        // Real: avaliar expressão recursivamente
        true
    }

    fn execute_insert(query: InsertQuery) -> Result<QueryResult, QueryError> {
        // 1. Validar schema
        // 2. Converter valores para CellValue
        // 3. Serializar row
        // 4. Inserir no StorageEngine via B-Tree
        // 5. Atualizar índices

        let rows_inserted = query.values.len();

        // Em produção:
        // for row_values in query.values {
        //     let row = Row { cells: convert_to_cell_values(row_values) };
        //     let key = generate_primary_key();
        //     storage.insert(key, row.encode());
        // }

        Ok(QueryResult::RowsAffected(rows_inserted))
    }

    fn execute_update(query: UpdateQuery) -> Result<QueryResult, QueryError> {
        // 1. Scan tabela
        // 2. Aplicar WHERE para filtrar rows
        // 3. Para cada row matched:
        //    - Atualizar campos especificados
        //    - Reescrever row no storage
        // 4. Atualizar índices

        let mut rows_affected = 0;

        // Simplified: assumir todas rows matchearam
        if query.where_clause.is_some() {
            rows_affected = 1; // Mock: 1 row afetada
        }

        // Em produção:
        // for (key, row) in storage.scan_table(query.table) {
        //     if eval_where(query.where_clause, row) {
        //         update_row(row, query.set);
        //         storage.update(key, row);
        //         rows_affected += 1;
        //     }
        // }

        Ok(QueryResult::RowsAffected(rows_affected))
    }

    fn execute_delete(query: DeleteQuery) -> Result<QueryResult, QueryError> {
        // 1. Scan tabela
        // 2. Aplicar WHERE para filtrar rows
        // 3. Para cada row matched:
        //    - Marcar como deleted (tombstone)
        //    - Adicionar ao WAL
        // 4. Atualizar índices

        let mut rows_affected = 0;

        // Simplified
        if query.where_clause.is_some() {
            rows_affected = 1; // Mock
        } else {
            rows_affected = 0; // DELETE sem WHERE é perigoso!
        }

        // Em produção:
        // for (key, row) in storage.scan_table(query.table) {
        //     if eval_where(query.where_clause, row) {
        //         storage.delete(key);
        //         rows_affected += 1;
        //     }
        // }

        Ok(QueryResult::RowsAffected(rows_affected))
    }

    fn execute_create_table(query: CreateTableQuery) -> Result<QueryResult, QueryError> {
        // 1. Validar schema (nomes únicos, tipos válidos)
        // 2. Criar metadata da tabela
        // 3. Alocar página root para B-Tree da tabela
        // 4. Persistir schema no catalog
        // 5. Criar índices se especificados

        // Validar que tem pelo menos uma coluna
        if query.columns.is_empty() {
            return Err(QueryError::ExecutionError(
                alloc::string::String::from("Table must have at least one column")
            ));
        }

        // Validar que nomes são únicos
        let mut seen_names = alloc::collections::BTreeSet::new();
        for col in &query.columns {
            if !seen_names.insert(&col.name) {
                return Err(QueryError::ExecutionError(
                    alloc::format!("Duplicate column name: {}", col.name)
                ));
            }
        }

        // Em produção:
        // let schema = TableSchema::from_create_query(query);
        // catalog.add_table(schema);
        // storage.alloc_table_btree();

        Ok(QueryResult::Success)
    }
}

/// Resultado de query
#[derive(Debug)]
pub enum QueryResult {
    /// Linhas retornadas (SELECT)
    Rows(Vec<Vec<Value>>),
    /// Número de linhas afetadas (INSERT/UPDATE/DELETE)
    RowsAffected(usize),
    /// Sucesso sem dados
    Success,
}

/// Erro de query
#[derive(Debug)]
pub enum QueryError {
    ParseError(String),
    ExecutionError(String),
    TableNotFound(String),
    ColumnNotFound(String),
}
