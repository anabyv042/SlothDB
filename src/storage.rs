mod buffer_pool;
mod disk_manager;
mod page;
mod pager;
pub mod record_manager;

pub const PAGE_SIZE: usize = 4096;

#[cfg(test)]
mod tests {
    use crate::{row::Row, storage::record_manager::RecordManager};
    use tempfile::tempdir;

    #[test]
    fn serializes_and_deserializes() {
        let dir = tempdir().unwrap();

        let mut rm = RecordManager::new(&dir.path().join("database.db"));

        let mut rows = vec![];
        for i in 0..1000000 {
            let row = Row {
                id: i,
                name: format!("name_{i}"),
                age: 10,
            };
            rm.insert_record(&row);
            rows.push(row.clone());
        }

        let scanned_rows = rm.scan_records();
        assert_eq!(rows, scanned_rows);
    }
}
