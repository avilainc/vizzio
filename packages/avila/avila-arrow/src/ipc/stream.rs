//! Arrow IPC stream format

use crate::error::Result;
use std::io::{Read, Write};

/// IPC stream writer
pub struct StreamWriter<W: Write> {
    writer: W,
}

impl<W: Write> StreamWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn write_batch(&mut self, _batch: &[u8]) -> Result<()> {
        // Placeholder
        Ok(())
    }
}

/// IPC stream reader
pub struct StreamReader<R: Read> {
    reader: R,
}

impl<R: Read> StreamReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn read_batch(&mut self) -> Result<Option<Vec<u8>>> {
        // Placeholder
        Ok(None)
    }
}
