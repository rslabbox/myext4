use myext4::spec::dir_entry::DirEntry;
use myext4::spec::extent::ExtentHeader;
use myext4::spec::group_desc::GroupDesc;
use myext4::spec::inode::Inode;
use myext4::spec::superblock::Superblock;

#[test]
fn spec_types_are_constructible() {
    let _dir = DirEntry;
    let _extent = ExtentHeader;
    let _group = GroupDesc;
    let _inode = Inode;
    let _sb = Superblock;
}

#[test]
fn spec_types_have_basic_memory_properties() {
    assert!(core::mem::size_of::<DirEntry>() <= 4096);
    assert!(core::mem::size_of::<ExtentHeader>() <= 4096);
    assert!(core::mem::size_of::<GroupDesc>() <= 4096);
    assert!(core::mem::size_of::<Inode>() <= 4096);
    assert!(core::mem::size_of::<Superblock>() <= 4096);

    assert!(core::mem::align_of::<DirEntry>() >= 1);
    assert!(core::mem::align_of::<ExtentHeader>() >= 1);
    assert!(core::mem::align_of::<GroupDesc>() >= 1);
    assert!(core::mem::align_of::<Inode>() >= 1);
    assert!(core::mem::align_of::<Superblock>() >= 1);
}
