use super::PAGE_SIZE;

pub struct RawPage {
    id: i32,
    data: Vec<u8>,
}

impl RawPage {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            data: vec![0; PAGE_SIZE],
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
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

pub struct RowPage {
    page: RawPage,
    free_space_offset: u16,
    slots: Vec<Option<u16>>,
}

impl RowPage {
    pub fn new(id: i32) -> Self {
        Self {
            page: RawPage::new(id),
            free_space_offset: 8,
            slots: Vec::new(),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        self.page.get_data()
    }

    pub fn insert_tuple(&mut self, tuple: &[u8]) -> Option<usize> {
        let tuple_size = tuple.len();
        let required_space = tuple_size + 2; // 2 bytes for slot metadata

        if self.free_space_offset as usize + required_space > PAGE_SIZE {
            return None; // Not enough space
        }

        // Find an empty slot or create a new one
        let slot_index = if let Some((i, _)) = self
            .slots
            .iter()
            .enumerate()
            .find(|(_, slot)| slot.is_none())
        {
            i
        } else {
            self.slots.push(None);
            self.slots.len() - 1
        };

        // Store tuple in free space
        let tuple_offset = self.free_space_offset;
        self.page.write(tuple_offset as usize, tuple);

        // Update slot directory
        self.slots[slot_index] = Some(tuple_offset);
        self.free_space_offset += tuple_size as u16;

        Some(slot_index)
    }
}
