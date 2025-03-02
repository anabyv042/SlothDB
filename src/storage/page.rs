use super::{
    tuple::{Tuple, TupleMetadata},
    PAGE_SIZE,
};
use crate::table::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawPage {
    id: usize,
    data: Vec<u8>,
}

impl RawPage {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            data: vec![0; PAGE_SIZE],
        }
    }

    pub fn from_bytes(id: usize, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn read(&self, offset: usize, length: usize) -> Option<&[u8]> {
        if offset + length <= PAGE_SIZE {
            Some(&self.data[offset..offset + length])
        } else {
            None
        }
    }

    pub fn write(&mut self, offset: usize, buf: &[u8]) -> anyhow::Result<()> {
        let len = buf.len();
        if offset + len <= PAGE_SIZE {
            self.data[offset..offset + len].copy_from_slice(buf);
        } else {
            anyhow::bail!("Write is out of bounds");
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Slot {
    offset: u16,
    size: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RowPage {
    page_id: u32,
    tuple_count: u16,
    free_space_offset: u16,
    slots: Vec<Slot>,
    data: Vec<u8>,
}

impl RowPage {
    pub fn new(id: u32) -> Self {
        Self {
            page_id: id,
            free_space_offset: 0,
            tuple_count: 0,
            slots: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn insert_row(&mut self, tuple: &Row) -> Option<usize> {
        let tuple_bytes = bincode::serialize(&tuple).unwrap();
        let tuple_size = tuple_bytes.len();
        let required_space = tuple_size + 2;

        if self.free_space_offset as usize + required_space > PAGE_SIZE {
            return None; // Not enough space
        }

        let slot_index = self.slots.len();
        let slot = Slot {
            offset: self.free_space_offset,
            size: tuple_bytes.len() as u16,
        };
        self.slots.push(slot);
        self.tuple_count += 1;

        // Store tuple in free space
        self.data.extend(&tuple_bytes);
        self.free_space_offset += tuple_size as u16;
        Some(slot_index)
    }

    /// Retrieves a tuple by slot index
    pub fn get_tuple(&self, slot_index: usize) -> Option<Row> {
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
        bytes[4..6].copy_from_slice(&self.tuple_count.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.free_space_offset.to_le_bytes());

        let mut offset = 8;
        for slot in &self.slots {
            bytes[offset..offset + 2].copy_from_slice(&slot.offset.to_le_bytes());
            bytes[offset + 2..offset + 4].copy_from_slice(&slot.size.to_le_bytes());
            offset += 4;
        }

        bytes[offset..offset+self.data.len()].copy_from_slice(&self.data);

        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        let page_id = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let tuple_count = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let free_space_offset = u16::from_le_bytes(bytes[6..8].try_into().unwrap());

        let mut offset = 8;
        let mut slots = vec![];
        for _ in 0..tuple_count {
            let tuple_offset = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
            let tuple_size = u16::from_le_bytes(bytes[offset + 2..offset + 4].try_into().unwrap());
            slots.push(Slot {
                offset: tuple_offset,
                size: tuple_size,
            });
            offset += 4;
        }
        let data = bytes[offset..offset+free_space_offset as usize].to_vec();

        Self {
            page_id,
            tuple_count,
            free_space_offset,
            slots,
            data,
        }
    }
}
