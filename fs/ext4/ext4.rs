// An implementation of the ext4 file system in Rust.
//
// Copyright 2018 Hayden Curfman
//
// Permission is hereby granted, free of charge, 
// to any person obtaining a copy of this software 
// and associated documentation files (the "Software"), 
// to deal in the Software without restriction, including 
// without limitation the rights to use, copy, modify, 
// merge, publish, distribute, sublicense, and/or sell 
// copies of the Software, and to permit persons to whom 
// the Software is furnished to do so, subject to the 
// following conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions 
// of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF 
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED 
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A 
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL 
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF 
// CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR 
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
// DEALINGS IN THE SOFTWARE.

extern crate core;
extern crate mish;

#![no_std]

mod memory;
mod error;

// common header for later structs
pub struct journal_log {
    buf: char
    blknr: i32
}

// TODO: Document offsets
pub struct ext4_super_block {
    s_inodes_count: i32,
    s_blocks_count_lo: i32,
    s_r_blocks_count_lo: i32,
    s_free_blocks_count_lo: i32,
    s_free_inodes_count: i32,
    s_first_data_block: i32,
    s_log_block_size: i32,
    s_log_cluster_size: i32,
    s_blocks_per_group: i32,
    s_clusters_per_group: i32,
    s_inodes_per_group: i32,
    s_mtime: i32,
    s_wtime: i32,
    s_mnt_count: i16,
    s_max_mnt_count: i16,
    s_magic: i16,
    s_state: i16,
    s_errors: i16,
    s_minor_rev_level: i16,
    s_lastcheck: i32,
    s_checkinterval: i32,
    s_creator_os: i32,
    s_rev_level: i32,
    s_def_resuid: i16,
    s_def_resgid: i16,
}

// TODO: Document offsets
pub struct ext2_dynamic_rev {
    s_first_ino: i32,
    s_inode_size: i16,
    s_block_group_nr: i16,
    s_feature_compat: i32,
    s_feature_incompat: i32,
    s_feature_ro_compat: i32,
    s_uuid[16]: u8,
    s_volume_name[16]: char,
    s_last_mounted[64]: char,
    s_algorithm_usage_bitmap: i32,
    // performance hints
    s_prealloc_blocks: u8,
    s_prealloc_dir_blocks: u8,
    s_reserved_gdt_blocks: i16,
    // journaling support valid if ext4_feature_compat_has_journal set
    s_journal_uuid[16]: u8,
    s_journal_inum: i32,
    s_journal_dev: i32,
    s_last_orphan: i32,
    s_hash_seed[4]: i32,
    s_def_hash_version: u8,
    s_jnl_backup_tpe: u8,
    s_desc_size: i16,
    s_default_mount_opts: i32,
    s_first_meta_bg: i32,
    s_mkfs_time: i32,
    s_jnl_blocks[17]: i32,
    // 64 bit support if ext4_feature_compat_64bit
    s_blocks_count_hi: i32,
    s_r_blocks_count_hi: i32,
    s_free_blocks_count_hi: i32,
    s_min_extra_isize: i16,
    s_want_extra_isize: i16,
    s_flags: i32,
    s_raid_stride: i16,
    s_mmp_interval: i16,
    s_mmp_block: i64,
    s_raid_stripe_width: i32,
    s_log_groups_per_flex: u8,
    s_checksum_type: u8,
    s_reserved_pad: i16,
    s_kbytes_written: i64,
    s_snapshot_inum: i32,
    s_snapshot_id: i32,
    s_snapshot_r_blocks_count: i64,
    s_snapshot_list: i32,
    s_error_count: i32,
    s_first_error_time: i32,
    s_first_error_ino: i32,
    s_first_error_block: i64,
    s_first_error_func[32]: u8,
    s_first_error_line: i32,
    s_last_error_time: i32,
    s_last_error_ino: i32,
    s_last_error_line: i32,
    s_last_error_block: i64,
    s_last_error_func[32]: u8,
    s_mount_opts[64]: u8,
    s_usr_quota_inum: i32,
    s_grp_quota_inum: i32,
    s_overhead_blocks: i32,
    s_backup_bgs[2]: i32,
    s_encrypt_algos[4]: u8,
    s_encrypt_pw_salt[16]: u8,
    s_lpf_ino: i32,
    s_prj_quota_inumL i32,
    s_checksum_seed: i32,
    s_reserved[98]: i32,
    s_checksum: i32,
}

pub struct ext4_group_desc {
    bg_block_bitmap_lo: i32,
    bg_inode_bitmap_lo: i32,
    bg_inode_table_lo: i32,
    bg_free_blocks_count_lo: i16,
    bg_free_inodes_count_lo: i16,
    bg_used_dirs_count_lo: i16,
    bg_flags: i16,
    bg_exclude_bitmap_lo: i16,
    bg_block_bitmap_csum_lo: i16,
    bg_inode_bitmap_csum_lo: i16,
    bg_itable_unused_lo: i16,
    bg_checksum: i16,
    // only if 64bit is enabled and s_desc_size > 32
    bg_block_bitmap_hi: i32,
    bg_inode_bitmap_hi: i32,
    bg_inode_table_hi: i32,
    bg_free_blocks_count_hi: i16,
    bg_free_inodes_count_hi: i16,
    bg_used_dirs_count_hi: i16,
    bg_itable_unused_hi: i16,
    bg_exclude_bitmap_hi: i32,
    bg_block_bitmap_csum_hi: i16,
    bg_inode_bitmap_csum_hi: i16,
    bg_reserved: u32,
}

pub struct ext4_inode {
    i_mode: i16,
    i_uid: i16,
    i_size_lo: i32,
    i_atime: i32,
    i_ctime: i32,
    i_mtime: i32,
    i_dtime: i32,
    i_gid: i16,
    i_links_count: i16,
    i_blocks_lo: i32,
    i_flags: i32,
    l_i_version: i32, // OFFSET: 0x0
    i_block[EXT4_N_BLOCKS=15]: i8, // OFFSET: 0X28
    i_generation: i32, // OFFSET: Ox64
    i_file_acl_lo: i32,
    i_size_high: i32,
    i_obso_faddr: i32,
    l_i_blocks_high: i16,
    l_i_file_acl_high: i16,
    l_i_uid_high: i16,
    l_i_gid_high: i16,
    l_i_checksum_lo: i16,
    l_i_reserved: i16,
    i_extra_isize: i16,
    i_checksum_hi: i16,
    i_ctime_extra: i32,
    i_mtime_extra: i32,
    i_atime_extra: i32,
    i_crtime: i32,
    i_crtime_extra: i32,
    i_version_hi: i32,
    i_projid: i32,
}

pub struct ext4_extent_header {
    eh_magic: i16
    eh_entries: i16
    eh_max: i16
    eh_depth: i16
    eh_generation: i16
}

pub struct ext4_extent_idx {
    ei_block: i32
    ei_leaf_lo: i32
    ei_leaf_hi: i16
    ei_unused: u16
}

pub struct ext4_extent {
    ee_block: i32
    ee_len: i16
    ee_start_hi: i16
    ee_start_lo: i32
}

pub struct ext4_extent_tail {
    eb_checksum: i32
}

pub struct ext4_dir_entry {
    inode: i32
    rec_len: i16
    name_len: i16
    name[EXT4_NAME_LEN]: char
}

pub struct ext4_dir_entry_2 {
    inode: i32
    rec_len: i16
    name_len: u8
    file_type: u8
    name[EXT4_NAME_LEN]: u8
}

pub struct ext4_dir_entry_tail {
    det_reserved_zero1: i32
    det_rec_len: i16
    det_reserved_zero2: u8
    det_reserved_ft: u8
    det_checksum: i32
}

pub struct dx_root {
    dot_inode: i32
    dot_rec_len: i16
    dot_name_len: u8
    dot_file_type: u8
    dot_name[4]: char
    dotdot_inode: i32
    dotdot_rec_len: i16
    dotdot_name_len: u8
    dotdot_file_type: u8
    dotdot_name[4]: char
    // TODO: Seperate -----
    reserved_zero: i32
    hash_version: u8
    info_length: u8
    indirect_levels: u8
    unused_flags: u8
    // --------------------
    limit: i16
    count: i16
    block: i32
    entries[0]: dx_entry
}

struct dx_entry {
    hash: i32
    block: i32
}

pub struct dx_tail {
    dt_reserved: u32
    dt_checksum: i32
}

pub struct ext4_xattr_ibody_header {
    h_magic: i32
}

pub struct ext4_xattr_header {
    h_magic: i32
    h_refcount: i32
    h_blocks: i32
    h_hash: i32
    h_checksum: i32
    h_reserved[2]: u32
}

pub struct ext4_xattr_entry {
    e_name_len: u8
    e_name_index: u8
    e_value_offs: i16
    e_value_inum: i32
    e_value_size: i32
    e_hash: i32
    e_name[e_name_len]: char
}

pub struct mmp_struct {
    mmp_magic: i32
    mmp_seq: i32
    mmp_time: i64
    mmp_nodename: str
    mmp_bdevname: str
    mmp_check_interval: i16
    mmp_pad1: i16
    mmp_pad2: i32
    mmp_checksum: i32
}

pub struct journal_header_s {
    h_magic: i32
    h_blocktype: i32
    h_sequence: i32
}

pub struct journal_superblock_s {
    s_header: journal_header_t
    s_blocksize: i32
    s_maxlen: i32
    s_first: i32
    s_sequence: i32
    s_start: i32
    s_errno: i32
    s_feature_compat: i32
    s_feature_incompat: i32
    s_feature_ro_compat: i32
    s_uuid[16]: u8
    s_nr_users: i32
    s_dynsuper: i32
    s_max_transaction: i32
    s_max_trans_data: i32
    s_checksum_type: u8
    s_padding2: u8
    s_padding[42]: u32
    s_checksum: i32
    s_users[16*48]: u8
}

pub struct journal_block_tag3_s {
    t_blocknr: i32
    t_flags: i32
    t_blocknr_high: i32
    t_checksum: i32
    uuid[16]: char
}

pub struct journal_block_tag_s {
    t_blocknr: i32
    t_checksum: i32
    t_flags: i32
    t_blocknr_high: i32
    uuid[16]: char
}

pub struct jbd2_journal_block_tail {
    t_checksum: i32
}

pub struct jbd2_journal_revoke_header_s {
    r_header: journal_header_t
    r_count: i32
    blocks[0]: i64
}

pub struct jbd2_journal_revoke_tail {
    r_checksum: i32
}

pub struct commit_header {
    h_header: journal_header_s
    h_chksum_type: u8
    h_chksum_size: u8
    h_padding[2]: u8
    h_chksum[JBD2_CHECKSUM_BYTES]: i32
    h_commit_sec: i64
    h_commit_nsec: i32
}
