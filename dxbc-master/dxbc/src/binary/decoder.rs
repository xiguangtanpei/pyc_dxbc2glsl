use super::error::Error;

use std::slice;
use std::str;

use byteorder::{ByteOrder, LittleEndian};

pub type DecoderResult<T> = Result<T, Error>;

pub struct Decoder<'a> {
    bytes: &'a [u8],
    offset: usize,
    limit: Option<usize>,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Decoder {
            bytes,
            offset: 0,
            limit: None,
        }
    }

    pub fn seek_mut(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn seek(&self, offset: usize) -> Self {
        Decoder {
            bytes: self.bytes,
            offset: offset,
            limit: self.limit
        }
    }

    pub fn eof(&self) -> bool {
        self.offset >= self.bytes.len()
    }

    pub fn scoped_decoder(&self, len: usize) -> Self {
        Decoder {
            bytes: &self.bytes[self.offset..(self.offset + len)],
            offset: 0,
            limit: None,
        }
    }

    pub fn skip(&mut self, n: usize) {
        self.offset += n;
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn bytes(&mut self, n: usize) -> &'a [u8] {
        let slice = &self.bytes[self.offset..(self.offset + n)];

        self.offset += n;

        slice
    }

    pub fn words(&mut self, n: usize) -> &'a [u32] {
        let byte_len = 4 * n;
        let slice = &self.bytes[self.offset..(self.offset + byte_len)];

        self.offset += byte_len;

        unsafe {
            slice::from_raw_parts(slice.as_ptr() as _, n)
        }
    }

    pub fn read_u64(&mut self) -> u64 {
        let val = LittleEndian::read_u64(&self.bytes[self.offset..]);
        self.offset += 8;
        val
    }

    pub fn read_u32(&mut self) -> u32 {
        let val = LittleEndian::read_u32(&self.bytes[self.offset..]);
        self.offset += 4;
        val
    }

    pub fn read_u32_address(&mut self) -> *const u32 {
        let ptr = self.bytes[self.offset..].as_ptr() as _;
        self.offset += 4;
        ptr
    }

    pub fn read_u16(&mut self) -> u16 {
        let val = LittleEndian::read_u16(&self.bytes[self.offset..]);
        self.offset += 2;
        val
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.bytes[self.offset];
        self.offset += 1;
        val
    }

    pub fn str(&mut self) -> DecoderResult<&'a str> {
        let null = self.bytes[self.offset..].iter().position(|&b| b == 0).unwrap_or(self.bytes.len());

        let string = str::from_utf8(
            &self.bytes[self.offset..(self.offset + null)]
        ).map_err(|e| Error::DecodeStrFailed(self.offset, e));

        self.offset += null + 1;

        string
    }

    pub fn string(&mut self) -> DecoderResult<String> {
        let null = self.bytes[self.offset..].iter().position(|&b| b == 0).unwrap_or(self.bytes.len());

        let string = String::from_utf8(
            self.bytes[self.offset..(self.offset + null)]
                .to_vec()
        ).map_err(|e| Error::DecodeStringFailed(self.offset, e));

        self.offset += null + 1;

        string
    }
}
