use super::PAGE_SIZE;
use crate::record::Record;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The page is full")]
    NotEnoughSpace,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Slot {
    offset: u16,
    size: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Page {
    page_id: u32,
    record_count: u16,
    free_space_offset: u16,
    slots: Vec<Slot>,
    data: Vec<u8>,
    // runtime data - won't be serialized
    pub is_dirty: bool,
    pub referenced_recently: bool,
}

impl Page {
    pub fn new(id: u32) -> Self {
        Self {
            page_id: id,
            free_space_offset: 0,
            record_count: 0,
            slots: Vec::new(),
            data: Vec::new(),
            is_dirty: false,
            referenced_recently: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.page_id
    }

    pub fn get_record_count(&self) -> usize {
        self.record_count as usize
    }

    pub fn is_enough_space(&self, record: &Record) -> bool {
        let record_bytes = bincode::serialize(&record).unwrap();
        let record_size = record_bytes.len();
        let required_space = record_size + 4;

        8 + self.record_count as usize * 4 + self.data.len() + required_space <= PAGE_SIZE
    }

    pub fn insert_record(&mut self, record: &Record) -> Result<usize, Error> {
        self.referenced_recently = true;
        let record_bytes = bincode::serialize(&record).unwrap();
        let record_size = record_bytes.len();
        let required_space = record_size + 4;

        if 8 + self.record_count as usize * 4 + self.data.len() + required_space > PAGE_SIZE {
            return Err(Error::NotEnoughSpace);
        }

        let slot_index = self.slots.len();
        let slot = Slot {
            offset: self.free_space_offset,
            size: record_bytes.len() as u16,
        };
        self.slots.push(slot);
        self.record_count += 1;

        // Store record in free space
        self.data.extend(&record_bytes);
        self.is_dirty = true;
        self.free_space_offset += record_size as u16;
        Ok(slot_index)
    }

    /// Retrieves a record by slot index
    pub fn read_record(&mut self, slot_index: usize) -> Option<Record> {
        self.referenced_recently = true;
        if let Some(slot) = self.slots.get(slot_index) {
            let offset = slot.offset as usize;
            let size = slot.size as usize;
            Some(bincode::deserialize(&self.data[offset..offset + size]).unwrap())
        } else {
            None
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![0; PAGE_SIZE];
        bytes[0..4].copy_from_slice(&self.page_id.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.record_count.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.free_space_offset.to_le_bytes());

        let mut offset = 8;
        for slot in &self.slots {
            bytes[offset..offset + 2].copy_from_slice(&slot.offset.to_le_bytes());
            bytes[offset + 2..offset + 4].copy_from_slice(&slot.size.to_le_bytes());
            offset += 4;
        }

        bytes[offset..offset + self.data.len()].copy_from_slice(&self.data);

        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        let page_id = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let record_count = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let free_space_offset = u16::from_le_bytes(bytes[6..8].try_into().unwrap());

        let mut offset = 8;
        let mut slots = vec![];
        for _ in 0..record_count {
            let record_offset = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
            let record_size = u16::from_le_bytes(bytes[offset + 2..offset + 4].try_into().unwrap());
            slots.push(Slot {
                offset: record_offset,
                size: record_size,
            });
            offset += 4;
        }
        let data = bytes[offset..offset + free_space_offset as usize].to_vec();

        Self {
            page_id,
            record_count: record_count,
            free_space_offset,
            is_dirty: false,
            referenced_recently: false,
            slots,
            data,
        }
    }
}
