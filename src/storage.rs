use catalog::Catalog;
use disk_manager::DiskManager;
use page::RowPage;
use tuple::{FieldType, FieldValue, Tuple, TupleMetadata};

use crate::table::Row;

mod buffer_pool;
pub mod catalog;
pub mod disk_manager;
pub mod page;
mod tuple;

pub const PAGE_SIZE: usize = 4096;

#[test]
fn test() {
    let mut disk_manager = DiskManager::new("database.db");

    let mut page = RowPage::new(1);
    let sample_row = Row {name: "name".to_string(), age: 10};
    let slot = page.insert_row(&sample_row).unwrap();

    disk_manager
        .write_page(1, &page.serialize())
        .expect("Failed to write page");

    let mut page_data: Vec<u8> = vec![0; PAGE_SIZE];
    disk_manager.read_page(1, page_data.as_mut_slice()).unwrap();
    let page = RowPage::deserialize(&page_data);

    let actual_tuple = page.get_tuple(slot).unwrap();

    assert_eq!(sample_row, actual_tuple);
}
