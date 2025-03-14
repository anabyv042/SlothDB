mod buffer_pool;
mod disk_manager;
mod page;
mod pager;
pub mod record_manager;

pub const PAGE_SIZE: usize = 4096;

#[cfg(test)]
mod tests {
    use crate::{record::Record, storage::record_manager::RecordManager};
    use tempfile::tempdir;

    #[test]
    fn records_spread_over_multiple_pages() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("large_db.db");

        let mut rm = RecordManager::new(&db_path);

        let num_rows = 5000;
        let mut rows = Vec::with_capacity(num_rows);

        for i in 0..num_rows {
            let row = Record {
                id: i as u32,
                name: format!("user_{}", i),
                age: (i % 120) as u8,
            };
            rm.insert_record(&row);
            rows.push(row);
        }

        // Verify that all rows are retrieved correctly
        let retrieved_rows: Vec<Record> = rm.scan_records().collect();
        assert_eq!(rows.len(), retrieved_rows.len());
        assert_eq!(rows, retrieved_rows);

        // Reopen the database and verify again
        drop(rm);
        let mut rm2 = RecordManager::new(&db_path);
        let retrieved_rows_2: Vec<Record> = rm2.scan_records().collect();
        assert_eq!(rows, retrieved_rows_2);
    }

    #[test]
    fn serializes_and_deserializes() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("database.db");
        let mut rm = RecordManager::new(&db_path);

        let records_to_insert = vec![
            Record { id: 1, name: "Alice".to_string(), age: 25 },
            Record { id: 2, name: "Bob".to_string(), age: 30 },
            Record { id: 3, name: "Charlie".to_string(), age: 35 },
        ];
        for record in &records_to_insert {
            rm.insert_record(record);
        }

        // scanned rows are equal to initial rows
        let scanned_rows: Vec<Record> = rm.scan_records().collect();
        assert_eq!(records_to_insert, scanned_rows);

        // reinitializing record manager to test consumption from the file
        drop(rm);
        let mut new_rm = RecordManager::new(&db_path);

        let scanned_rows: Vec<Record> = new_rm.scan_records().collect();
        assert_eq!(records_to_insert, scanned_rows);
    }
}
