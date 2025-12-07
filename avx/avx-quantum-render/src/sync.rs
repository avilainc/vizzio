//! Thread-safe synchronization wrappers for concurrent rendering

use std::sync::{Arc, RwLock, Mutex};
use crate::scene::Scene;
use crate::renderer::QEDRenderer;
use crate::errors::RenderResult;

/// Thread-safe scene wrapper for concurrent access
pub struct SafeScene {
    inner: Arc<RwLock<Scene>>,
}

impl SafeScene {
    /// Create a new thread-safe scene
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Scene::new())),
        }
    }

    /// Execute a read operation on the scene
    pub fn read<F, T>(&self, f: F) -> RenderResult<T>
    where
        F: FnOnce(&Scene) -> T,
    {
        let scene = self.inner.read()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire read lock".to_string()))?;
        Ok(f(&scene))
    }

    /// Execute a write operation on the scene
    pub fn write<F, T>(&self, f: F) -> RenderResult<T>
    where
        F: FnOnce(&mut Scene) -> T,
    {
        let mut scene = self.inner.write()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire write lock".to_string()))?;
        Ok(f(&mut scene))
    }

    /// Clone the scene for independent use
    pub fn clone_scene(&self) -> RenderResult<Scene> {
        let scene = self.inner.read()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire read lock".to_string()))?;
        Ok(scene.clone())
    }
}

impl Default for SafeScene {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SafeScene {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Render job for the job queue
pub struct RenderJob {
    /// Job ID
    pub id: usize,
    /// Tile x coordinate
    pub tile_x: usize,
    /// Tile y coordinate
    pub tile_y: usize,
    /// Tile width
    pub width: usize,
    /// Tile height
    pub height: usize,
    /// Samples per pixel
    pub spp: usize,
}

/// Thread-safe render job queue
pub struct RenderQueue {
    jobs: Arc<Mutex<Vec<RenderJob>>>,
}

impl RenderQueue {
    /// Create a new render queue
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Enqueue a render job
    pub fn enqueue(&self, job: RenderJob) -> RenderResult<()> {
        let mut jobs = self.jobs.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire queue lock".to_string()))?;
        jobs.push(job);
        Ok(())
    }

    /// Dequeue the next render job
    pub fn dequeue(&self) -> RenderResult<Option<RenderJob>> {
        let mut jobs = self.jobs.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire queue lock".to_string()))?;
        Ok(jobs.pop())
    }

    /// Get queue length
    pub fn len(&self) -> RenderResult<usize> {
        let jobs = self.jobs.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire queue lock".to_string()))?;
        Ok(jobs.len())
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> RenderResult<bool> {
        let jobs = self.jobs.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to acquire queue lock".to_string()))?;
        Ok(jobs.is_empty())
    }
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RenderQueue {
    fn clone(&self) -> Self {
        Self {
            jobs: Arc::clone(&self.jobs),
        }
    }
}

/// Metrics for concurrent rendering
#[derive(Debug, Clone, Default)]
pub struct ConcurrentMetrics {
    /// Total tiles processed
    pub tiles_processed: usize,
    /// Total samples accumulated
    pub samples: usize,
    /// Render time in seconds
    pub render_time: f64,
    /// Average time per tile
    pub avg_tile_time: f64,
}

/// Thread-safe metrics accumulator
pub struct MetricsAccumulator {
    metrics: Arc<Mutex<ConcurrentMetrics>>,
}

impl MetricsAccumulator {
    /// Create a new metrics accumulator
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(ConcurrentMetrics::default())),
        }
    }

    /// Record a tile completion
    pub fn record_tile(&self, samples: usize, tile_time: f64) -> RenderResult<()> {
        let mut m = self.metrics.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to lock metrics".to_string()))?;
        m.tiles_processed += 1;
        m.samples += samples;
        m.render_time += tile_time;
        if m.tiles_processed > 0 {
            m.avg_tile_time = m.render_time / m.tiles_processed as f64;
        }
        Ok(())
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> RenderResult<ConcurrentMetrics> {
        let m = self.metrics.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to lock metrics".to_string()))?;
        Ok(m.clone())
    }

    /// Reset metrics
    pub fn reset(&self) -> RenderResult<()> {
        let mut m = self.metrics.lock()
            .map_err(|_| crate::errors::RenderError::ThreadError("Failed to lock metrics".to_string()))?;
        *m = ConcurrentMetrics::default();
        Ok(())
    }
}

impl Default for MetricsAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MetricsAccumulator {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_scene_creation() {
        let scene = SafeScene::new();
        assert!(scene.read(|_| true).is_ok());
    }

    #[test]
    fn test_render_queue_enqueue_dequeue() {
        let queue = RenderQueue::new();
        let job = RenderJob {
            id: 1,
            tile_x: 0,
            tile_y: 0,
            width: 64,
            height: 64,
            spp: 100,
        };

        assert!(queue.enqueue(job).is_ok());
        assert_eq!(queue.len().unwrap(), 1);
        assert!(queue.dequeue().is_ok());
        assert!(queue.is_empty().unwrap());
    }

    #[test]
    fn test_metrics_accumulator() {
        let acc = MetricsAccumulator::new();

        // Record some tiles
        assert!(acc.record_tile(100, 0.5).is_ok());
        assert!(acc.record_tile(100, 0.5).is_ok());

        let metrics = acc.get_metrics().unwrap();
        assert_eq!(metrics.tiles_processed, 2);
        assert_eq!(metrics.samples, 200);
        assert!((metrics.render_time - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_safe_scene_clone() {
        let scene1 = SafeScene::new();
        let scene2 = scene1.clone();

        // Both should be able to access the same underlying scene
        assert!(scene1.read(|_| true).is_ok());
        assert!(scene2.read(|_| true).is_ok());
    }

    #[test]
    fn test_render_queue_multiple_jobs() {
        let queue = RenderQueue::new();

        for i in 0..10 {
            let job = RenderJob {
                id: i,
                tile_x: i,
                tile_y: i,
                width: 32,
                height: 32,
                spp: 50,
            };
            assert!(queue.enqueue(job).is_ok());
        }

        assert_eq!(queue.len().unwrap(), 10);

        // Dequeue all
        for _ in 0..10 {
            assert!(queue.dequeue().is_ok());
        }

        assert!(queue.is_empty().unwrap());
    }
}
