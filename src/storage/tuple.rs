use std::{collections::HashMap, mem::size_of};

#[derive(Clone)]
pub enum FieldType {
    Integer,
    Text(usize),
}

impl FieldType {
    pub fn size(&self) -> usize {
        match self {
            FieldType::Integer => size_of::<i32>(),
            FieldType::Text(s) => s * size_of::<u8>(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    Integer(i32),
    Text(String),
}

#[derive(Clone)]
pub struct TupleMetadata {
    columns: Vec<String>,
    types: Vec<FieldType>,
}

impl TupleMetadata {
    pub fn size(&self) -> usize {
        let mut size = 0;
        for t in &self.types {
            size += t.size();
        }
        size
    }
}

pub struct Tuple {
    metadata: TupleMetadata,
    data: Vec<FieldValue>,
}

impl Tuple {
    pub fn from_bytes(metadata: TupleMetadata, bytes: &[u8]) -> Self {
        if bytes.len() != metadata.size() {
            panic!("Invalid size of bytes");
        }
        let mut data = vec![];
        let mut offset = 0;
        for t in &metadata.types {
            let value_size = t.size();
            let value = match t {
                FieldType::Integer => {
                    let val_array: [u8; 4] = bytes[offset..offset + value_size]
                        .try_into()
                        .expect("Failed to extract bytes for integer value");
                    let val = i32::from_le_bytes(val_array);
                    FieldValue::Integer(val)
                }
                FieldType::Text(_) => {
                    let b = bytes[offset..offset + value_size].to_vec();
                    let end_index = b.iter().position(|&x| x == 0).unwrap_or(bytes.len());
                    let val: String = String::from_utf8(b[..end_index].to_vec())
                        .expect("Failed to extract bytes for string value");
                    FieldValue::Text(val)
                }
            };
            offset += value_size;
            data.push(value);
        }
        Self { metadata, data }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0..self.data.len() {
            let v = &self.data[i];
            let t = &self.metadata.types[i];

            match v {
                FieldValue::Integer(i) => bytes.extend(i.to_le_bytes()),
                FieldValue::Text(s) => {
                    let mut padded_bytes = vec![0u8; t.size()];
                    let ut8_bytes = s.as_bytes();
                    padded_bytes[..ut8_bytes.len()].copy_from_slice(&ut8_bytes);
                    bytes.extend(padded_bytes)
                }
            }
        }
        bytes
    }
}

#[test]
fn serializes_and_deserializes() {
    let metadata = TupleMetadata {
        columns: vec!["column1".to_string(), "column2".to_string()],
        types: vec![FieldType::Integer, FieldType::Text(20)],
    };
    let data = vec![
        FieldValue::Integer(3),
        FieldValue::Text("test_text".to_string()),
    ];
    let tuple = Tuple {
        metadata: metadata.clone(),
        data,
    };

    let bytes = tuple.to_bytes();
    let parsed_tuple = Tuple::from_bytes(metadata, &bytes);

    assert_eq!(parsed_tuple.data, tuple.data);
}
