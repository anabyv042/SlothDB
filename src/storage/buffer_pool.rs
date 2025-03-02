use super::{disk_manager::DiskManager, page::Page, PAGE_SIZE};
use std::collections::HashMap;

pub const DEFAULT_CAPACITY: usize = 10;

pub struct BufferPool {
    cache: HashMap<u32, Page>,
    capacity: usize,
    clock_hand: Option<usize>,
}

impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: HashMap::new(),
            capacity,
            clock_hand: None,
        }
    }

    pub fn read_page(&mut self, page_id: u32, disk_manager: &mut DiskManager) -> Option<&mut Page> {
        if !self.cache.contains_key(&page_id) {
            if self.cache.len() == self.capacity {
                if let Some(victim) = self.find_victim() {
                    self.evict(victim, disk_manager);
                }
            }

            let mut buffer = vec![0; PAGE_SIZE];
            disk_manager.read_page(page_id, &mut buffer).unwrap();
            let page = Page::deserialize(&buffer);
            self.cache.insert(page_id, page);
        }
        return self.cache.get_mut(&page_id);
    }

    pub fn evict(&mut self, page_id: u32, disk_manager: &mut DiskManager) {
        if let Some(page) = self.cache.remove(&page_id) {
            if page.is_dirty {
                disk_manager.write_page(page_id, &page.serialize()).unwrap();
            }
        }
    }

    pub fn flush(&mut self, disk_manager: &mut DiskManager) {
        for page in self.cache.values() {
            if page.is_dirty {
                disk_manager
                    .write_page(page.get_id(), &page.serialize())
                    .unwrap();
            }
        }
    }

    fn find_victim(&mut self) -> Option<u32> {
        if self.cache.len() < self.capacity {
            return None;
        }

        let mut pages: Vec<u32> = self.cache.values().map(|p| p.get_id()).collect();
        pages.sort();

        let mut clock_hand = self.clock_hand.unwrap_or(0);

        for _ in 0..self.capacity * 2 {
            let page_id = pages[clock_hand];

            if let Some(page) = self.cache.get_mut(&page_id) {
                match page.referenced_recently {
                    true => {
                        page.referenced_recently = false;
                    }
                    false => {
                        let victim = page_id;
                        self.clock_hand = Some((clock_hand + 1) % self.cache.len());
                        return Some(victim);
                    }
                }
            }

            // Advance clock handle circularly
            clock_hand = (clock_hand + 1) % self.cache.len();
        }

        None
    }
}
