//! Connection pooling for reusing TCP/TLS connections

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Connection pool key (host + port)
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PoolKey {
    pub host: String,
    pub port: u16,
}

/// Pooled connection with metadata
pub struct PooledConnection<T> {
    pub connection: T,
    pub created_at: Instant,
    pub last_used: Instant,
}

/// Connection pool manager
pub struct ConnectionPool<T> {
    pools: Arc<Mutex<HashMap<PoolKey, Vec<PooledConnection<T>>>>>,
    max_idle_per_host: usize,
    max_idle_time: Duration,
}

impl<T> ConnectionPool<T> {
    pub fn new(max_idle_per_host: usize, max_idle_time: Duration) -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            max_idle_per_host,
            max_idle_time,
        }
    }

    /// Get a connection from the pool
    pub fn get(&self, key: &PoolKey) -> Option<T> {
        let mut pools = self.pools.lock().unwrap();
        if let Some(connections) = pools.get_mut(key) {
            // Remove expired connections
            connections.retain(|conn| {
                conn.last_used.elapsed() < self.max_idle_time
            });

            // Return the most recently used connection
            connections.pop().map(|conn| conn.connection)
        } else {
            None
        }
    }

    /// Return a connection to the pool
    pub fn put(&self, key: PoolKey, connection: T) {
        let mut pools = self.pools.lock().unwrap();
        let connections = pools.entry(key).or_insert_with(Vec::new);

        if connections.len() < self.max_idle_per_host {
            connections.push(PooledConnection {
                connection,
                created_at: Instant::now(),
                last_used: Instant::now(),
            });
        }
    }

    /// Clear all connections from pool
    pub fn clear(&self) {
        let mut pools = self.pools.lock().unwrap();
        pools.clear();
    }
}

impl<T> Clone for ConnectionPool<T> {
    fn clone(&self) -> Self {
        Self {
            pools: Arc::clone(&self.pools),
            max_idle_per_host: self.max_idle_per_host,
            max_idle_time: self.max_idle_time,
        }
    }
}
