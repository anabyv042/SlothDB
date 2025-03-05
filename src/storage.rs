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
    fn serializes_and_deserializes() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("database.db");

        let mut rm = RecordManager::new(&db_path);

        let mut rows = vec![];
        for i in 0..1000 {
            let row = Record {
                id: i,
                name: format!("name_{i}"),
                age: 10,
            };
            rm.insert_record(&row);
            rows.push(row.clone());
        }

        // scanned rows are equal to initial rows
        let scanned_rows = rm.scan_records();
        assert_eq!(rows, scanned_rows);

        // reinitializing record manager to test consumption from the file
        drop(rm);
        let mut new_rm = RecordManager::new(&db_path);

        let scanned_rows = new_rm.scan_records();
        assert_eq!(rows, scanned_rows);
    }
}
