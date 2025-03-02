use super::tuple::TupleMetadata;
use std::collections::HashMap;

pub struct Catalog {
    id_counter: u32,
    name_to_id: HashMap<String, u32>,
    tables: HashMap<u32, TupleMetadata>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            tables: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, name: String, tuple_metadata: TupleMetadata) -> u32 {
        let id = self.id_counter;
        self.id_counter += 1;
        self.tables.insert(id, tuple_metadata);
        self.name_to_id.insert(name, id);
        id
    }

    pub fn get_tuple_metadata(&self, table_id: u32) -> Option<&TupleMetadata> {
        self.tables.get(&table_id)
    }
}
