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

    pub fn scan_records(&mut self) -> Vec<Record> {
        let mut rows = vec![];
        for page_id in 0..self.page_count {
            let page = self.pager.read_page(page_id).unwrap();
            for slot in 0..page.get_record_count() {
                if let Some(row) = page.read_record(slot) {
                    rows.push(row);
                }
            }
        }
        rows
    }
}
