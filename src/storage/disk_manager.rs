use std::fs::{File, Metadata, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use super::PAGE_SIZE;

pub struct DiskManager {
    file: File,
}

impl DiskManager {
    /// Opens an existing file or creates a new one.
    pub fn new(file_path: &Path) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open database file");

        Self { file }
    }

    /// Reads a page from disk into a buffer.
    pub fn read_page(&mut self, page_id: u32, buffer: &mut [u8]) -> std::io::Result<()> {
        let offset = (page_id as u64) * (PAGE_SIZE as u64);
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(buffer)?;
        Ok(())
    }

    /// Writes a page buffer to disk at the given page ID.
    pub fn write_page(&mut self, page_id: u32, buffer: &[u8]) -> std::io::Result<()> {
        let offset = (page_id as u64) * (PAGE_SIZE as u64);
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(buffer)?;
        self.file.flush()?; // Ensure data is written to disk
        Ok(())
    }

    pub fn get_page_count(&self) -> usize {
        Metadata::len(&self.file.metadata().unwrap()) as usize / PAGE_SIZE
    }
}
