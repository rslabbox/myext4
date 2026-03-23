use std::fs;

const DISK_IMAGE: &str = "tests/disk.img";
const SUPERBLOCK_OFFSET: usize = 1024;
const EXT4_MAGIC_OFFSET_IN_SB: usize = 0x38;
const EXT4_MAGIC: u16 = 0xEF53;

fn read_u16_le(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([buf[offset], buf[offset + 1]])
}

fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]])
}

#[test]
fn disk_image_exists_and_is_large_enough() {
    let meta = fs::metadata(DISK_IMAGE).expect("tests/disk.img should exist");
    assert!(meta.is_file(), "tests/disk.img must be a file");
    assert!(
        meta.len() as usize >= SUPERBLOCK_OFFSET + 1024,
        "image must contain at least one superblock"
    );
}

#[test]
fn disk_image_contains_ext4_magic_in_superblock() {
    let bytes = fs::read(DISK_IMAGE).expect("failed to read tests/disk.img");
    let magic_off = SUPERBLOCK_OFFSET + EXT4_MAGIC_OFFSET_IN_SB;
    let magic = read_u16_le(&bytes, magic_off);
    assert_eq!(magic, EXT4_MAGIC, "superblock magic should be 0xEF53");
}

#[test]
fn disk_image_superblock_core_fields_are_non_zero() {
    let bytes = fs::read(DISK_IMAGE).expect("failed to read tests/disk.img");
    let sb = &bytes[SUPERBLOCK_OFFSET..SUPERBLOCK_OFFSET + 1024];

    // ext4 base fields: inodes_count @ 0x00, blocks_count_lo @ 0x04
    let inodes_count = read_u32_le(sb, 0x00);
    let blocks_count_lo = read_u32_le(sb, 0x04);
    let log_block_size = read_u32_le(sb, 0x18);

    assert!(inodes_count > 0, "inodes_count should be non-zero");
    assert!(blocks_count_lo > 0, "blocks_count_lo should be non-zero");

    // Valid ext block size is 1024 << log_block_size, usually <= 64KiB.
    let block_size = 1024u64 << log_block_size;
    assert!(
        (1024..=65536).contains(&block_size),
        "block size out of expected range: {}",
        block_size
    );
}
