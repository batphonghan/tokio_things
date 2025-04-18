use std::error::Error;
use std::io::{self, Cursor, Read, Write};

#[derive(Debug)]
pub struct Data {
    pub field1: u32,
    pub field2: i16,
    pub field3: String,
}

impl Data {
    pub fn serialize(&self) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::new();

        let _ = bytes.write(&self.field1.to_ne_bytes())?;
        let _ = bytes.write(&self.field2.to_ne_bytes())?;
        let field3_len = self.field3.len() as u32;
        let _ = bytes.write(&field3_len.to_ne_bytes())?;
        bytes.extend_from_slice(self.field3.as_bytes());

        Ok(bytes)
    }

    pub fn deserialize(cursor: &mut Cursor<&[u8]>) -> io::Result<Data> {
        let mut field1_bytes = [0u8; 4];
        let mut field2_bytes = [0u8; 2];

        cursor.read_exact(&mut field1_bytes)?;
        cursor.read_exact(&mut field2_bytes)?;

        let field1 = u32::from_ne_bytes(field1_bytes);
        let field2 = i16::from_ne_bytes(field2_bytes);

        let mut len_byte = [0u8; 4];
        cursor.read_exact(&mut len_byte)?;
        let field3_len = u32::from_ne_bytes(len_byte);

        let mut field3_bytes = vec![0u8; field3_len as usize];
        cursor.read_exact(&mut field3_bytes)?;
        let field3 = String::from_utf8(field3_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        Ok(Data {
            field1,
            field2,
            field3,
        })
    }
}
