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

#[cfg(test)]
mod tests {
    use crate::{record::Record, storage::record_manager::RecordManager};
    use tempfile::tempdir;

    #[test]
    fn test_record_manager_insert_and_scan() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_db.db");
        let mut rm = RecordManager::new(&db_path);

        let records_to_insert = vec![
            Record { id: 1, name: "Alice".to_string(), age: 25 },
            Record { id: 2, name: "Bob".to_string(), age: 30 },
            Record { id: 3, name: "Charlie".to_string(), age: 35 },
        ];
        for record in &records_to_insert {
            rm.insert_record(record);
        }

        let scanned_records = rm.scan_records();
        assert_eq!(scanned_records, records_to_insert);
    }

    #[test]
    fn test_record_manager_persistence() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_db.db");

        // Create a RecordManager, insert records, and drop it (simulating closing the database)
        {
            let mut rm = RecordManager::new(&db_path);
            let records_to_insert = vec![
                Record { id: 1, name: "Alice".to_string(), age: 25 },
                Record { id: 2, name: "Bob".to_string(), age: 30 },
            ];
            for record in &records_to_insert {
                rm.insert_record(record);
            }
        }

        // Create a new RecordManager instance (simulating reopening the database)
        let mut rm = RecordManager::new(&db_path);

        let scanned_records = rm.scan_records();
        assert_eq!(
            scanned_records,
            vec![
                Record { id: 1, name: "Alice".to_string(), age: 25 },
                Record { id: 2, name: "Bob".to_string(), age: 30 },
            ]
        );
    }
}
