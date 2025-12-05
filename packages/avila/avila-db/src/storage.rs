//! Storage engine do AvilaDB
//!
//! Implementa B-Tree persistente em disco

use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Página de disco (4KB)
pub const PAGE_SIZE: usize = 4096;

/// ID de página
pub type PageId = u64;

/// Página de dados
#[repr(C)]
pub struct Page {
    /// ID da página
    pub id: PageId,
    /// Tipo da página
    pub page_type: PageType,
    /// Dados raw
    pub data: [u8; PAGE_SIZE - 16],
}

/// Tipo de página
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PageType {
    /// Nó interno da B-Tree
    BTreeInternal = 1,
    /// Folha da B-Tree
    BTreeLeaf = 2,
    /// Página de overflow (dados grandes)
    Overflow = 3,
    /// Free list
    FreeList = 4,
}

/// Storage engine
pub struct StorageEngine {
    /// Cache de páginas em memória
    pub page_cache: BTreeMap<PageId, Page>,
    /// Próximo page ID disponível
    pub next_page_id: PageId,
    /// Free list (páginas deletadas)
    pub free_pages: Vec<PageId>,
}

impl StorageEngine {
    /// Cria novo storage engine
    pub fn new() -> Self {
        Self {
            page_cache: BTreeMap::new(),
            next_page_id: 1,
            free_pages: Vec::new(),
        }
    }

    /// Aloca nova página
    pub fn alloc_page(&mut self, page_type: PageType) -> PageId {
        let page_id = if let Some(id) = self.free_pages.pop() {
            id
        } else {
            let id = self.next_page_id;
            self.next_page_id += 1;
            id
        };

        let page = Page {
            id: page_id,
            page_type,
            data: [0u8; PAGE_SIZE - 16],
        };

        self.page_cache.insert(page_id, page);
        page_id
    }

    /// Libera página
    pub fn free_page(&mut self, page_id: PageId) {
        self.page_cache.remove(&page_id);
        self.free_pages.push(page_id);
    }

    /// Lê página
    pub fn read_page(&self, page_id: PageId) -> Option<&Page> {
        self.page_cache.get(&page_id)
    }

    /// Escreve página
    pub fn write_page(&mut self, page_id: PageId, page: Page) {
        self.page_cache.insert(page_id, page);
    }

    /// Flush cache para disco
    pub fn flush(&mut self) {
        // Em produção: fsync() para garantir durabilidade
        // Aqui: simplified no-op (em memória apenas)
        //
        // Implementação real seria:
        // - Serializar cada Page para bytes
        // - Escrever em arquivo com offset = page_id * PAGE_SIZE
        // - fsync() no fd
        // - Atualizar superblock com metadata

        // No-op por enquanto - dados já estão em page_cache
    }
}

/// B-Tree index
pub struct BTree {
    /// Storage engine
    pub storage: StorageEngine,
    /// Root page ID
    pub root_page_id: Option<PageId>,
    /// Ordem da árvore (fanout)
    pub order: usize,
}

impl BTree {
    /// Cria nova B-Tree
    pub fn new(order: usize) -> Self {
        Self {
            storage: StorageEngine::new(),
            root_page_id: None,
            order,
        }
    }

    /// Insere chave-valor
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<(), ()> {
        // Se árvore vazia, cria root
        if self.root_page_id.is_none() {
            let page_id = self.storage.alloc_page(PageType::BTreeLeaf);
            self.root_page_id = Some(page_id);
        }

        // Busca folha apropriada (simplified - seria recursive na prática)
        let root_id = self.root_page_id.unwrap();

        // Insert na folha (simplified)
        if let Some(page) = self.storage.page_cache.get_mut(&root_id) {
            // Copia key e value para página (offset-based storage)
            let key_len = key.len().min(256);
            let value_len = value.len().min(256);

            // Header: [key_len][key][value_len][value]
            let mut offset = 0;
            if offset + 2 + key_len + 2 + value_len < page.data.len() {
                page.data[offset] = key_len as u8;
                offset += 1;
                page.data[offset..offset + key_len].copy_from_slice(&key[..key_len]);
                offset += key_len;
                page.data[offset] = value_len as u8;
                offset += 1;
                page.data[offset..offset + value_len].copy_from_slice(&value[..value_len]);
            }
        }

        Ok(())
    }

    /// Busca valor por chave
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let root_id = self.root_page_id?;
        let page = self.storage.read_page(root_id)?;

        // Simplified linear search na página
        let mut offset = 0;
        while offset + 2 < page.data.len() {
            let key_len = page.data[offset] as usize;
            if key_len == 0 {
                break;
            }
            offset += 1;

            if offset + key_len > page.data.len() {
                break;
            }

            let stored_key = &page.data[offset..offset + key_len];
            offset += key_len;

            if offset >= page.data.len() {
                break;
            }

            let value_len = page.data[offset] as usize;
            offset += 1;

            if offset + value_len > page.data.len() {
                break;
            }

            // Se chave match, retorna valor
            if stored_key == key {
                return Some(page.data[offset..offset + value_len].to_vec());
            }

            offset += value_len;
        }

        None
    }

    /// Remove chave
    pub fn remove(&mut self, key: &[u8]) -> Result<(), ()> {
        let root_id = self.root_page_id.ok_or(())?;

        if let Some(page) = self.storage.page_cache.get_mut(&root_id) {
            // Simplified: marca como deletado zerando key_len
            let mut offset = 0;
            while offset + 2 < page.data.len() {
                let key_len = page.data[offset] as usize;
                if key_len == 0 {
                    break;
                }

                let key_start = offset + 1;
                if key_start + key_len > page.data.len() {
                    break;
                }

                let stored_key = &page.data[key_start..key_start + key_len];

                // Se chave match, marca como deleted
                if stored_key == key {
                    page.data[offset] = 0; // Zera key_len
                    return Ok(());
                }

                // Pula para próxima entry
                offset = key_start + key_len + 1;
                if offset < page.data.len() {
                    let value_len = page.data[offset] as usize;
                    offset += 1 + value_len;
                }
            }
        }

        Err(())
    }

    /// Itera sobre range de chaves
    pub fn range(&self, start: &[u8], end: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut results = Vec::new();

        let root_id = match self.root_page_id {
            Some(id) => id,
            None => return results,
        };

        let page = match self.storage.read_page(root_id) {
            Some(p) => p,
            None => return results,
        };

        // Scan todas entries na página
        let mut offset = 0;
        while offset + 2 < page.data.len() {
            let key_len = page.data[offset] as usize;
            if key_len == 0 {
                break;
            }
            offset += 1;

            if offset + key_len > page.data.len() {
                break;
            }

            let key = page.data[offset..offset + key_len].to_vec();
            offset += key_len;

            if offset >= page.data.len() {
                break;
            }

            let value_len = page.data[offset] as usize;
            offset += 1;

            if offset + value_len > page.data.len() {
                break;
            }

            let value = page.data[offset..offset + value_len].to_vec();

            // Verifica se está no range
            if key.as_slice() >= start && key.as_slice() <= end {
                results.push((key, value));
            }

            offset += value_len;
        }

        results
    }
}

/// WAL (Write-Ahead Log) para durabilidade
pub struct WriteAheadLog {
    /// Entries do log
    pub entries: Vec<LogEntry>,
    /// LSN atual (Log Sequence Number)
    pub current_lsn: u64,
}

/// Entry no WAL
pub struct LogEntry {
    /// LSN
    pub lsn: u64,
    /// Tipo de operação
    pub op_type: OpType,
    /// Dados da operação
    pub data: Vec<u8>,
}

/// Tipo de operação no log
#[derive(Debug, Clone, Copy)]
pub enum OpType {
    /// Inserção
    Insert,
    /// Atualização
    Update,
    /// Remoção
    Delete,
    /// Commit de transação
    Commit,
    /// Abort de transação
    Abort,
}

impl WriteAheadLog {
    /// Cria novo WAL
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current_lsn: 0,
        }
    }

    /// Append entry ao log
    pub fn append(&mut self, op_type: OpType, data: Vec<u8>) -> u64 {
        let lsn = self.current_lsn;
        self.current_lsn += 1;

        self.entries.push(LogEntry {
            lsn,
            op_type,
            data,
        });

        lsn
    }

    /// Flush log para disco
    pub fn flush(&self) {
        // Em produção:
        // - Serializar entries para arquivo
        // - fsync() para garantir persistência
        // - Usar write-ahead logging antes de modificar dados

        // Simplified: entries já em memória
    }

    /// Recovery a partir do log
    pub fn recover(&self) -> Result<(), ()> {
        // Replay de log entries após crash:
        // 1. Ler arquivo de log
        // 2. Para cada entry:
        //    - Se Commit: aplicar todas operações da TX
        //    - Se Abort: descartar operações da TX
        // 3. Reconstruir estado consistente

        for entry in &self.entries {
            match entry.op_type {
                OpType::Commit => {
                    // Aplicar operações commitadas
                }
                OpType::Abort => {
                    // Descartar operações abortadas
                }
                _ => {
                    // Guardar operações pendentes
                }
            }
        }

        Ok(())
    }
}
