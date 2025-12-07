//! Streaming buffer with backpressure support

use alloc::vec::Vec;
use alloc::collections::VecDeque;
use avila_error::{Error, ErrorKind, Result};

pub struct StreamBuffer {
    buffer: VecDeque<u8>,
    capacity: usize,
    high_watermark: usize,
    low_watermark: usize,
    blocked: bool,
}

impl StreamBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        let high = (capacity * 3) / 4;
        let low = capacity / 4;

        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            high_watermark: high,
            low_watermark: low,
            blocked: false,
        }
    }

    pub fn set_watermarks(&mut self, low: usize, high: usize) -> Result<()> {
        if low >= high || high > self.capacity {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid watermarks"));
        }
        self.low_watermark = low;
        self.high_watermark = high;
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        if self.blocked {
            return Ok(0);
        }

        let available = self.capacity - self.buffer.len();
        let to_write = available.min(data.len());

        for &byte in &data[..to_write] {
            self.buffer.push_back(byte);
        }

        if self.buffer.len() >= self.high_watermark {
            self.blocked = true;
        }

        Ok(to_write)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let to_read = buf.len().min(self.buffer.len());

        for i in 0..to_read {
            buf[i] = self.buffer.pop_front().unwrap();
        }

        if self.blocked && self.buffer.len() <= self.low_watermark {
            self.blocked = false;
        }

        to_read
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn available_space(&self) -> usize {
        self.capacity - self.buffer.len()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.blocked = false;
    }
}

pub struct RateLimiter {
    tokens: f64,
    max_tokens: f64,
    rate: f64,
    last_update: u64,
}

impl RateLimiter {
    pub fn new(rate: f64, burst: f64) -> Self {
        Self {
            tokens: burst,
            max_tokens: burst,
            rate,
            last_update: 0,
        }
    }

    pub fn try_acquire(&mut self, tokens: f64, now: u64) -> bool {
        self.refill(now);

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    fn refill(&mut self, now: u64) {
        if now > self.last_update {
            let elapsed = (now - self.last_update) as f64;
            self.tokens = (self.tokens + elapsed * self.rate).min(self.max_tokens);
            self.last_update = now;
        }
    }

    pub fn available(&self) -> f64 {
        self.tokens
    }
}

pub struct BatchBuffer {
    items: Vec<Vec<u8>>,
    batch_size: usize,
    current_size: usize,
}

impl BatchBuffer {
    pub fn new(batch_size: usize) -> Self {
        Self {
            items: Vec::new(),
            batch_size,
            current_size: 0,
        }
    }

    pub fn push(&mut self, item: Vec<u8>) -> Option<Vec<Vec<u8>>> {
        self.current_size += item.len();
        self.items.push(item);

        if self.current_size >= self.batch_size {
            self.flush()
        } else {
            None
        }
    }

    pub fn flush(&mut self) -> Option<Vec<Vec<u8>>> {
        if self.items.is_empty() {
            return None;
        }

        let items = core::mem::take(&mut self.items);
        self.current_size = 0;
        Some(items)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn total_size(&self) -> usize {
        self.current_size
    }
}

pub struct PriorityBuffer {
    high: VecDeque<Vec<u8>>,
    normal: VecDeque<Vec<u8>>,
    low: VecDeque<Vec<u8>>,
}

impl PriorityBuffer {
    pub fn new() -> Self {
        Self {
            high: VecDeque::new(),
            normal: VecDeque::new(),
            low: VecDeque::new(),
        }
    }

    pub fn push(&mut self, data: Vec<u8>, priority: Priority) {
        match priority {
            Priority::High => self.high.push_back(data),
            Priority::Normal => self.normal.push_back(data),
            Priority::Low => self.low.push_back(data),
        }
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.high.pop_front()
            .or_else(|| self.normal.pop_front())
            .or_else(|| self.low.pop_front())
    }

    pub fn len(&self) -> usize {
        self.high.len() + self.normal.len() + self.low.len()
    }

    pub fn is_empty(&self) -> bool {
        self.high.is_empty() && self.normal.is_empty() && self.low.is_empty()
    }

    pub fn clear(&mut self) {
        self.high.clear();
        self.normal.clear();
        self.low.clear();
    }
}

impl Default for PriorityBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    High,
    Normal,
    Low,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_buffer() {
        let mut buf = StreamBuffer::with_capacity(10);

        let written = buf.write(b"12345").unwrap();
        assert_eq!(written, 5);

        let mut output = vec![0u8; 3];
        let read = buf.read(&mut output);
        assert_eq!(read, 3);
        assert_eq!(&output, b"123");
    }

    #[test]
    fn test_backpressure() {
        let mut buf = StreamBuffer::with_capacity(10);
        buf.set_watermarks(2, 8).unwrap();

        buf.write(b"12345678").unwrap();
        assert!(buf.is_blocked());

        let mut output = vec![0u8; 7];
        buf.read(&mut output);
        assert!(!buf.is_blocked());
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(1.0, 10.0);

        assert!(limiter.try_acquire(5.0, 0));
        assert_eq!(limiter.available(), 5.0);

        limiter.refill(10);
        assert_eq!(limiter.available(), 10.0);
    }

    #[test]
    fn test_batch_buffer() {
        let mut batch = BatchBuffer::new(10);

        assert!(batch.push(vec![1, 2, 3]).is_none());
        assert!(batch.push(vec![4, 5, 6]).is_none());

        let flushed = batch.push(vec![7, 8, 9, 10]);
        assert!(flushed.is_some());
        assert_eq!(flushed.unwrap().len(), 3);
    }

    #[test]
    fn test_priority_buffer() {
        let mut prio = PriorityBuffer::new();

        prio.push(vec![1], Priority::Low);
        prio.push(vec![2], Priority::High);
        prio.push(vec![3], Priority::Normal);

        assert_eq!(prio.pop().unwrap(), vec![2]);
        assert_eq!(prio.pop().unwrap(), vec![3]);
        assert_eq!(prio.pop().unwrap(), vec![1]);
    }
}
