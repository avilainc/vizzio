//! GPU Instancing para objetos repetidos

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

/// Identificador único de mesh (baseado em vértices/índices)
pub type MeshId = u64;

/// Instância de um objeto
#[derive(Debug, Clone)]
pub struct Instance {
    /// Índice da geometria original
    pub geometry_index: usize,
    /// Matriz de transformação (4x4)
    pub transform: [f32; 16],
    /// Cor da instância
    pub color: [f32; 4],
}

/// Batch de instâncias (mesmo mesh)
#[derive(Debug)]
pub struct InstanceBatch {
    /// ID do mesh
    pub mesh_id: MeshId,
    /// Vértices compartilhados
    pub vertices: Vec<[f32; 3]>,
    /// Índices compartilhados
    pub indices: Vec<u32>,
    /// Normais compartilhadas
    pub normals: Vec<[f32; 3]>,
    /// Lista de instâncias
    pub instances: Vec<Instance>,
}

/// Sistema de instancing
#[derive(Debug)]
pub struct InstancingSystem {
    /// Batches agrupados por mesh_id
    batches: BTreeMap<MeshId, InstanceBatch>,
}

impl InstancingSystem {
    /// Cria sistema vazio
    pub fn new() -> Self {
        Self {
            batches: BTreeMap::new(),
        }
    }

    /// Adiciona geometria e retorna mesh_id
    pub fn add_geometry(
        &mut self,
        geometry_index: usize,
        vertices: &[[f32; 3]],
        indices: &[u32],
        normals: &[[f32; 3]],
        color: [f32; 4],
    ) -> MeshId {
        let mesh_id = compute_mesh_id(vertices, indices);

        let batch = self.batches.entry(mesh_id).or_insert_with(|| {
            InstanceBatch {
                mesh_id,
                vertices: vertices.to_vec(),
                indices: indices.to_vec(),
                normals: normals.to_vec(),
                instances: Vec::new(),
            }
        });

        batch.instances.push(Instance {
            geometry_index,
            transform: identity_matrix(),
            color,
        });

        mesh_id
    }

    /// Retorna batches para renderização
    pub fn batches(&self) -> impl Iterator<Item = &InstanceBatch> {
        self.batches.values()
    }

    /// Número de batches
    pub fn batch_count(&self) -> usize {
        self.batches.len()
    }

    /// Número total de instâncias
    pub fn instance_count(&self) -> usize {
        self.batches.values().map(|b| b.instances.len()).sum()
    }

    /// Limpa todos os batches
    pub fn clear(&mut self) {
        self.batches.clear();
    }
}

/// Computa ID único do mesh (hash simplificado)
fn compute_mesh_id(vertices: &[[f32; 3]], indices: &[u32]) -> MeshId {
    let mut hash: u64 = 0;

    // Hash baseado no número de vértices e índices
    hash = hash.wrapping_mul(31).wrapping_add(vertices.len() as u64);
    hash = hash.wrapping_mul(31).wrapping_add(indices.len() as u64);

    // Amostra alguns vértices para evitar colisões
    if !vertices.is_empty() {
        hash = hash.wrapping_mul(31).wrapping_add((vertices[0][0] * 1000.0) as u64);
        hash = hash.wrapping_mul(31).wrapping_add((vertices[0][1] * 1000.0) as u64);
        hash = hash.wrapping_mul(31).wrapping_add((vertices[0][2] * 1000.0) as u64);
    }

    if vertices.len() > 1 {
        let mid = vertices.len() / 2;
        hash = hash.wrapping_mul(31).wrapping_add((vertices[mid][0] * 1000.0) as u64);
    }

    hash
}

/// Matriz identidade 4x4
fn identity_matrix() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instancing_system() {
        let mut system = InstancingSystem::new();

        let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let indices = vec![0, 1, 2];
        let normals = vec![[0.0, 0.0, 1.0]; 3];
        let color = [1.0, 0.0, 0.0, 1.0];

        // Adiciona mesma geometria 3 vezes
        system.add_geometry(0, &vertices, &indices, &normals, color);
        system.add_geometry(1, &vertices, &indices, &normals, color);
        system.add_geometry(2, &vertices, &indices, &normals, color);

        // Deve ter apenas 1 batch com 3 instâncias
        assert_eq!(system.batch_count(), 1);
        assert_eq!(system.instance_count(), 3);
    }

    #[test]
    fn test_different_geometries() {
        let mut system = InstancingSystem::new();

        let vertices1 = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let vertices2 = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        let indices = vec![0, 1];
        let normals = vec![[0.0, 0.0, 1.0]; 2];
        let color = [1.0, 0.0, 0.0, 1.0];

        system.add_geometry(0, &vertices1, &indices, &normals, color);
        system.add_geometry(1, &vertices2, &indices, &normals, color);

        // Geometrias diferentes = 2 batches
        assert_eq!(system.batch_count(), 2);
        assert_eq!(system.instance_count(), 2);
    }
}
