use storage::{disk_manager::DiskManager, page::RowPage};

mod storage;

#[test]
fn test() {
    let mut disk_manager = DiskManager::new("database.db");
    let mut page = RowPage::new(1);

    // Insert sample tuple
    let sample_tuple = vec![1, 2, 3, 4, 5, 6, 7, 8];
    page.insert_tuple(&sample_tuple);

    disk_manager
        .write_page(1, page.get_data())
        .expect("Failed to write page");
}
