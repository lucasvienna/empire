use std::io::{Read, Write};
use std::{fmt, mem, ptr};

use crate::models::error::{EmpResult, ErrorKind};

pub struct Buffer {
    data: Vec<u8>, // buffer data
    size: usize,   // size of buffer data (bytes)
    index: usize,  // index of next byte to be read
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut vec = Vec::with_capacity(2048);
        vec.resize(2048, 0);
        Buffer {
            data: vec,
            size: 0,
            index: 0,
        }
    }

    pub fn from(data: Vec<u8>) -> Buffer {
        let size = data.len();
        Buffer {
            data,
            size,
            index: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_write_data(&self) -> &[u8] {
        &self.data[0..self.size]
    }

    pub fn reset_read(&mut self) {
        self.index = 0;
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "bytes ({:?})", self.data)
    }
}

impl Write for Buffer {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        if len == 0 {
            return Ok(buf.len());
        }
        unsafe {
            ptr::copy(&buf[0], &mut self.data[self.size], buf.len());
        }
        self.size += len;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        if len == 0 {
            return Ok(buf.len());
        }
        unsafe {
            ptr::copy(&self.data[self.index], &mut buf[0], buf.len());
        }
        self.index += len;
        Ok(len)
    }
}

pub fn write_string(buffer: &mut Buffer, value: &str) -> std::io::Result<usize> {
    let bytes = value.as_bytes();
    let len = bytes.len();
    write_short(buffer, &(len as u16))?;
    buffer.write(bytes)
}
pub fn write_long(buffer: &mut Buffer, value: &u64) -> std::io::Result<usize> {
    assert!(buffer.index + 8 <= buffer.data.len());
    buffer.write(unsafe { &mem::transmute::<u64, [u8; 8]>(value.to_le()) })
}
pub fn write_integer(buffer: &mut Buffer, value: &u32) -> std::io::Result<usize> {
    assert!(buffer.index + 4 <= buffer.data.len());
    buffer.write(unsafe { &mem::transmute::<u32, [u8; 4]>(value.to_le()) })
}
pub fn write_short(buffer: &mut Buffer, value: &u16) -> std::io::Result<usize> {
    assert!(buffer.index + 2 <= buffer.data.len());
    buffer.write(unsafe { &mem::transmute::<u16, [u8; 2]>(value.to_le()) })
}
pub fn write_byte(buffer: &mut Buffer, value: &u8) -> std::io::Result<usize> {
    assert!(buffer.index + 1 <= buffer.data.len());
    buffer.write(&[*value])
}

pub fn read_string(buffer: &mut Buffer) -> EmpResult<String> {
    let len = read_short(buffer)? as usize;
    let value = String::from_utf8_lossy(&buffer.data[buffer.index..buffer.index + len]);
    buffer.index += len;
    Ok(value.to_string())
}
pub fn read_long(buffer: &mut Buffer) -> EmpResult<u64> {
    let bytes: &mut [u8; 8] = &mut [0, 0, 0, 0, 0, 0, 0, 0];
    try_read!(buffer.read(bytes), bytes.len());
    Ok(u64::from_le(unsafe {
        mem::transmute::<[u8; 8], u64>(*bytes)
    }))
}
pub fn read_integer(buffer: &mut Buffer) -> EmpResult<u32> {
    let bytes: &mut [u8; 4] = &mut [0, 0, 0, 0];
    try_read!(buffer.read(bytes), bytes.len());
    Ok(u32::from_le(unsafe {
        mem::transmute::<[u8; 4], u32>(*bytes)
    }))
}
pub fn read_short(buffer: &mut Buffer) -> EmpResult<u16> {
    let bytes: &mut [u8; 2] = &mut [0, 0];
    try_read!(buffer.read(bytes), bytes.len());
    Ok(u16::from_le(unsafe {
        mem::transmute::<[u8; 2], u16>(*bytes)
    }))
}
pub fn read_byte(buffer: &mut Buffer) -> EmpResult<u8> {
    let bytes: &mut [u8; 1] = &mut [0];
    try_read!(buffer.read(bytes), bytes.len());
    Ok(bytes[0])
}

#[test]
fn test_write_byte() {
    let mut buffer = Buffer::new();
    let value = 12u8;
    write_byte(&mut buffer, &value).unwrap();

    assert_eq!(buffer.get_write_data(), [12]);

    write_byte(&mut buffer, &value).unwrap();

    let read_value = read_byte(&mut buffer).unwrap();
    assert_eq!(read_value, value);
    assert_eq!(buffer.index, 1);
    assert_eq!(buffer.size, 2);
}
