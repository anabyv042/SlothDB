use std::path::Path;

use super::{
    buffer_pool::{BufferPool, DEFAULT_CAPACITY},
    disk_manager::DiskManager,
    page::Page,
};

pub struct Pager {
    page_count: usize,
    buffer_pool: BufferPool,
    disk_manager: DiskManager,
}

impl Pager {
    pub fn new(file_path: &Path, buffer_pool_capacity: Option<usize>) -> Self {
        let disk_manager = DiskManager::new(file_path);
        Self {
            page_count: disk_manager.get_page_count(),
            disk_manager: disk_manager,
            buffer_pool: BufferPool::new(buffer_pool_capacity.unwrap_or(DEFAULT_CAPACITY)),
        }
    }

    pub fn allocate_page(&mut self) -> u32 {
        let page_id = self.page_count as u32;
        self.page_count += 1;

        let page = Page::new(page_id);
        self.disk_manager
            .write_page(page_id, &page.serialize())
            .unwrap();

        page_id
    }

    pub fn read_page(&mut self, page_id: u32) -> Option<&mut Page> {
        self.buffer_pool.read_page(page_id, &mut self.disk_manager)
    }

    pub fn get_page_count(&self) -> usize {
        self.page_count
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        self.buffer_pool.flush(&mut self.disk_manager);
    }
}
