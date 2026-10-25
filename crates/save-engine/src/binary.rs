use crate::error::SaveError;

pub struct SaveBuffer {
    data: Vec<u8>,
    pub little_endian: bool,
}

impl SaveBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        SaveBuffer {
            data,
            little_endian: false,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    fn check_bounds(&self, offset: usize, size: usize) -> Result<(), SaveError> {
        if offset + size > self.data.len() {
            return Err(SaveError::Truncated { offset, len: self.data.len() });
        }
        Ok(())
    }

    pub fn read_u8(&self, offset: usize) -> Result<u8, SaveError> {
        self.check_bounds(offset, 1)?;
        Ok(self.data[offset])
    }

    pub fn write_u8(&mut self, offset: usize, val: u8) -> Result<(), SaveError> {
        self.check_bounds(offset, 1)?;
        self.data[offset] = val;
        Ok(())
    }

    pub fn read_u32(&self, offset: usize) -> Result<u32, SaveError> {
        self.check_bounds(offset, 4)?;
        let b = [
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ];
        Ok(if self.little_endian {
            u32::from_le_bytes(b)
        } else {
            u32::from_be_bytes(b)
        })
    }

    pub fn write_u32(&mut self, offset: usize, val: u32) -> Result<(), SaveError> {
        self.check_bounds(offset, 4)?;
        let bytes = if self.little_endian {
            val.to_le_bytes()
        } else {
            val.to_be_bytes()
        };
        self.data[offset..offset + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn read_f32(&self, offset: usize) -> Result<f32, SaveError> {
        Ok(f32::from_bits(self.read_u32(offset)?))
    }

    pub fn write_f32(&mut self, offset: usize, val: f32) -> Result<(), SaveError> {
        self.write_u32(offset, val.to_bits())
    }

    pub fn read_i32(&self, offset: usize) -> Result<i32, SaveError> {
        Ok(self.read_u32(offset)? as i32)
    }

    pub fn write_i32(&mut self, offset: usize, val: i32) -> Result<(), SaveError> {
        self.write_u32(offset, val as u32)
    }

    /// Reads a 64-bit value as two consecutive 32-bit reads, low word first, mirroring the
    /// source's `Variable.joinUInt64(readU32(o), readU32(o+4))`. TOTK is the only user of this
    /// today (horse amiibo UID); it's placed here rather than in `totk/` because it's a generic
    /// buffer primitive, same as `read_f32`.
    pub fn read_u64(&self, offset: usize) -> Result<u64, SaveError> {
        let lower = self.read_u32(offset)? as u64;
        let upper = self.read_u32(offset + 4)? as u64;
        Ok((upper << 32) | lower)
    }

    pub fn write_u64(&mut self, offset: usize, val: u64) -> Result<(), SaveError> {
        self.write_u32(offset, (val & 0xffffffff) as u32)?;
        self.write_u32(offset + 4, (val >> 32) as u32)
    }

    /// Reads `len` raw bytes starting at `offset`, used for variable-length binary blobs
    /// (TOTK AutoBuild's `CombinedActorInfo`), unlike every fixed-width primitive above.
    pub fn read_bytes(&self, offset: usize, len: usize) -> Result<Vec<u8>, SaveError> {
        self.check_bounds(offset, len)?;
        Ok(self.data[offset..offset + len].to_vec())
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> Result<(), SaveError> {
        self.check_bounds(offset, bytes.len())?;
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    /// Grows the buffer by inserting `additional_bytes` zero bytes at `offset`, shifting
    /// everything at or after `offset` later. The only buffer-growing operation in this crate;
    /// every other method assumes a fixed-size buffer, and continues to work unchanged
    /// afterward since `check_bounds` reads `self.data.len()` fresh on every call.
    pub fn insert_bytes(&mut self, offset: usize, additional_bytes: usize) -> Result<(), SaveError> {
        if offset > self.data.len() {
            return Err(SaveError::Truncated { offset, len: self.data.len() });
        }
        self.data.splice(offset..offset, std::iter::repeat(0u8).take(additional_bytes));
        Ok(())
    }

    /// Mirrors the source project's `MarcFile.readString`: consecutive non-null bytes
    /// starting at `offset`, stopping at the first null byte, `max_len`, or the end of
    /// the buffer.
    pub fn read_string(&self, offset: usize, max_len: usize) -> String {
        let mut out = String::new();
        for i in 0..max_len {
            let pos = offset + i;
            if pos >= self.data.len() {
                break;
            }
            let byte = self.data[pos];
            if byte == 0 {
                break;
            }
            out.push(byte as char);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_writes_u32_big_endian() {
        let mut buf = SaveBuffer::new(vec![0u8; 8]);
        buf.write_u32(0, 0x11223344).unwrap();
        assert_eq!(buf.data, vec![0x11, 0x22, 0x33, 0x44, 0, 0, 0, 0]);
        assert_eq!(buf.read_u32(0).unwrap(), 0x11223344);
    }

    #[test]
    fn reads_and_writes_u32_little_endian() {
        let mut buf = SaveBuffer::new(vec![0u8; 8]);
        buf.little_endian = true;
        buf.write_u32(0, 0x11223344).unwrap();
        assert_eq!(buf.data, vec![0x44, 0x33, 0x22, 0x11, 0, 0, 0, 0]);
        assert_eq!(buf.read_u32(0).unwrap(), 0x11223344);
    }

    #[test]
    fn read_string_stops_at_null_byte() {
        let buf = SaveBuffer::new(vec![b'H', b'i', 0, b'X']);
        assert_eq!(buf.read_string(0, 4), "Hi");
    }

    #[test]
    fn read_string_stops_at_max_len() {
        let buf = SaveBuffer::new(vec![b'a', b'b', b'c', b'd']);
        assert_eq!(buf.read_string(0, 2), "ab");
    }

    #[test]
    fn f32_round_trips_through_bits() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        buf.write_f32(0, 3.5).unwrap();
        assert_eq!(buf.read_f32(0).unwrap(), 3.5);
    }

    #[test]
    fn out_of_bounds_read_returns_truncated_error_instead_of_panicking() {
        let buf = SaveBuffer::new(vec![0u8; 4]);
        assert_eq!(buf.read_u32(1), Err(SaveError::Truncated { offset: 1, len: 4 }));
        assert_eq!(buf.read_u8(4), Err(SaveError::Truncated { offset: 4, len: 4 }));
    }

    #[test]
    fn out_of_bounds_write_returns_truncated_error_instead_of_panicking() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        assert_eq!(buf.write_u32(1, 0x11223344), Err(SaveError::Truncated { offset: 1, len: 4 }));
        assert_eq!(buf.write_u8(4, 1), Err(SaveError::Truncated { offset: 4, len: 4 }));
    }

    #[test]
    fn i32_round_trips_negative_values() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        buf.write_i32(0, -1).unwrap();
        assert_eq!(buf.read_i32(0).unwrap(), -1);
    }

    #[test]
    fn read_bytes_and_write_bytes_round_trip() {
        let mut buf = SaveBuffer::new(vec![0u8; 8]);
        buf.write_bytes(2, &[1, 2, 3, 4]).unwrap();
        assert_eq!(buf.read_bytes(2, 4).unwrap(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn read_bytes_out_of_bounds_returns_truncated_error() {
        let buf = SaveBuffer::new(vec![0u8; 4]);
        assert_eq!(buf.read_bytes(2, 4), Err(SaveError::Truncated { offset: 2, len: 4 }));
    }

    #[test]
    fn u64_round_trips_as_two_u32_words_low_word_first() {
        let mut buf = SaveBuffer::new(vec![0u8; 8]);
        buf.write_u64(0, 0x1122334455667788).unwrap();
        assert_eq!(buf.read_u32(0).unwrap(), 0x55667788); // low word
        assert_eq!(buf.read_u32(4).unwrap(), 0x11223344); // high word
        assert_eq!(buf.read_u64(0).unwrap(), 0x1122334455667788);
    }

    #[test]
    fn insert_bytes_grows_buffer_and_shifts_trailing_data() {
        let mut buf = SaveBuffer::new(vec![1, 2, 3, 4, 5]);
        buf.insert_bytes(2, 3).unwrap();
        assert_eq!(buf.into_bytes(), vec![1, 2, 0, 0, 0, 3, 4, 5]);
    }

    #[test]
    fn insert_bytes_at_end_appends_zeros() {
        let mut buf = SaveBuffer::new(vec![1, 2, 3]);
        buf.insert_bytes(3, 2).unwrap();
        assert_eq!(buf.into_bytes(), vec![1, 2, 3, 0, 0]);
    }

    #[test]
    fn insert_bytes_out_of_bounds_offset_returns_truncated_error() {
        let mut buf = SaveBuffer::new(vec![1, 2, 3]);
        let result = buf.insert_bytes(10, 4);
        assert_eq!(result, Err(SaveError::Truncated { offset: 10, len: 3 }));
    }
}
