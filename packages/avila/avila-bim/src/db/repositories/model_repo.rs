//! Model repository

use crate::db::models::*;
use uuid::Uuid;

pub struct ModelRepository {
    // TODO: Add sqlx::PgPool
}

impl ModelRepository {
    pub fn new() -> Self {
        Self {}
    }

    /// Criar novo modelo
    pub async fn create(&self, model: &DbModel) -> Result<DbModel, sqlx::Error> {
        // TODO: INSERT INTO models ...
        Ok(model.clone())
    }

    /// Buscar modelo por ID
    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<DbModel>, sqlx::Error> {
        // TODO: SELECT * FROM models WHERE id = $1
        Ok(None)
    }

    /// Atualizar status
    pub async fn update_status(&self, id: &Uuid, status: ModelStatus) -> Result<(), sqlx::Error> {
        // TODO: UPDATE models SET status = $1, updated_at = NOW() WHERE id = $2
        Ok(())
    }

    /// Atualizar erro
    pub async fn update_error(&self, id: &Uuid, error: &str) -> Result<(), sqlx::Error> {
        // TODO: UPDATE models SET status = 'error', error_message = $1 WHERE id = $2
        Ok(())
    }

    /// Listar modelos de um projeto
    pub async fn list_by_project(&self, project_id: &Uuid) -> Result<Vec<DbModel>, sqlx::Error> {
        // TODO: SELECT * FROM models WHERE project_id = $1 ORDER BY created_at DESC
        Ok(vec![])
    }
}

impl Default for ModelRepository {
    fn default() -> Self {
        Self::new()
    }
}
