use super::pager::Pager;
use crate::record::Record;
use std::path::Path;

pub struct RecordManager {
    pager: Pager,
    page_count: u32,
}

impl RecordManager {
    pub fn new(file_path: &Path) -> Self {
        let pager = Pager::new(file_path, None);
        Self {
            page_count: pager.get_page_count() as u32,
            pager,
        }
    }

    pub fn insert_record(&mut self, record: &Record) {
        if self.page_count == 0
            || !self
                .pager
                .read_page(self.page_count - 1)
                .unwrap()
                .is_enough_space(record)
        {
            self.pager.allocate_page();
            self.page_count += 1;
        }
        let page = self.pager.read_page(self.page_count - 1).unwrap();
        page.insert_record(&record).unwrap();
    }

    pub fn scan_records(&mut self) -> RecordIterator {
        RecordIterator {
            record_manager: self,
            current_page: 0,
            current_slot: 0,
        }
    }
}

pub struct RecordIterator<'a> {
    record_manager: &'a mut RecordManager,
    current_page: u32,
    current_slot: usize,
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_page < self.record_manager.page_count {
            let page = self
                .record_manager
                .pager
                .read_page(self.current_page)
                .unwrap();
            if self.current_slot < page.get_record_count() {
                let record = page.read_record(self.current_slot);
                self.current_slot += 1;
                return record;
            } else {
                self.current_page += 1;
                self.current_slot = 0;
            }
        }
        None
    }
}
