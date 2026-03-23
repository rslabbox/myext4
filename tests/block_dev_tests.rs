use std::cell::RefCell;

use myext4::block_dev::file_impl::FileBlockDevice;
use myext4::block_dev::BlockDevice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MockError {
    OutOfRange,
    WrongBufferSize,
}

struct MockBlockDevice {
    block_size: u32,
    blocks: RefCell<Vec<Vec<u8>>>,
}

impl MockBlockDevice {
    fn new(block_size: u32, block_count: usize) -> Self {
        Self {
            block_size,
            blocks: RefCell::new(vec![vec![0; block_size as usize]; block_count]),
        }
    }
}

impl BlockDevice for MockBlockDevice {
    type Error = MockError;

    fn read_block(&self, block_index: u64, buf: &mut [u8]) -> Result<(), Self::Error> {
        if buf.len() != self.block_size as usize {
            return Err(MockError::WrongBufferSize);
        }
        let blocks = self.blocks.borrow();
        let src = blocks
            .get(block_index as usize)
            .ok_or(MockError::OutOfRange)?;
        buf.copy_from_slice(src);
        Ok(())
    }

    fn write_block(&self, block_index: u64, buf: &[u8]) -> Result<(), Self::Error> {
        if buf.len() != self.block_size as usize {
            return Err(MockError::WrongBufferSize);
        }
        let mut blocks = self.blocks.borrow_mut();
        let dst = blocks
            .get_mut(block_index as usize)
            .ok_or(MockError::OutOfRange)?;
        dst.copy_from_slice(buf);
        Ok(())
    }

    fn block_size(&self) -> u32 {
        self.block_size
    }
}

#[test]
fn block_device_read_write_roundtrip() {
    let dev = MockBlockDevice::new(512, 4);
    let mut write_buf = vec![0u8; 512];
    for (idx, byte) in write_buf.iter_mut().enumerate() {
        *byte = (idx % 251) as u8;
    }

    dev.write_block(2, &write_buf).expect("write should succeed");

    let mut read_buf = vec![0u8; 512];
    dev.read_block(2, &mut read_buf).expect("read should succeed");

    assert_eq!(read_buf, write_buf);
}

#[test]
fn block_device_rejects_wrong_buffer_sizes() {
    let dev = MockBlockDevice::new(1024, 2);
    let mut small_read = vec![0u8; 512];
    let small_write = vec![0u8; 512];

    let read_err = dev
        .read_block(0, &mut small_read)
        .expect_err("small read buffer should fail");
    let write_err = dev
        .write_block(0, &small_write)
        .expect_err("small write buffer should fail");

    assert_eq!(read_err, MockError::WrongBufferSize);
    assert_eq!(write_err, MockError::WrongBufferSize);
}

#[test]
fn block_device_checks_index_bounds() {
    let dev = MockBlockDevice::new(256, 1);
    let mut read_buf = vec![0u8; 256];
    let write_buf = vec![1u8; 256];

    let read_err = dev
        .read_block(9, &mut read_buf)
        .expect_err("out-of-range read should fail");
    let write_err = dev
        .write_block(9, &write_buf)
        .expect_err("out-of-range write should fail");

    assert_eq!(read_err, MockError::OutOfRange);
    assert_eq!(write_err, MockError::OutOfRange);
}

#[test]
fn file_block_device_type_is_accessible() {
    let _placeholder = FileBlockDevice;
    assert_eq!(core::mem::size_of::<FileBlockDevice>(), 0);
}
