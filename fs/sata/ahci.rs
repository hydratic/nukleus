#![no_std]

extern crate volatile;

use volatile::Volatile;

mod ext4;
mod memory;

// pub const SATA_SIG_ATA: Volatile<u32> = 0x00000101;
// pub const SATA_SIG_ATAPI: Volatile<u32> = 0xEB140101;
// pub const SATA_SIG_SEMB: Volatile<u32> = 0xC33C0101;
// pub const SATA_SIG_PM: Volatile<u32> = 0x96690101;

// pub const AHCI_BASE: Volatile<u32> = 0x400000;
pub const AHCI_DEV_NULL: i8 = 0;
pub const AHCI_DEV_SATA: i8 = 1;
pub const AHCI_DEV_SEMB: i8 = 2;
pub const AHCI_DEV_PM: i8 = 3;
pub const AHCI_DEV_SATAPI: i8 = 4;

// pub const HBA_PxCMD_ST: Volatile<u32> = 0x0001;
// pub const HBA_PxCMD_FRE: Volatile<u32> = 0x0010;
// pub const HBA_PxCMD_FR: Volatile<u32> = 0x4000;
// pub const HBA_PxCMD_CR: Volatile<u32> = 0x8000;
pub const HBA_PORT_IPM_ACTIVE: i8 = 1;
pub const HBA_PORT_DET_PRESENT: i8 = 3;

enum FIS_TYPE {
    FIS_TYPE_REG_H2D,
    FIS_TYPE_REG_D2H,
    FIS_TYPE_DMA_ACT,
    FIS_TYPE_DMA_SETUP,
    FIS_TYPE_DATA,
    FIS_TYPE_BIST,
    FIS_TYPE_PIO_SETUP,
    FIS_TYPE_DEV_BITS,
}

pub struct FIS_REG_H2D {
    // DWORD 0
    fis_type: u8,

    pmport: u8,
    rsv0: u8,
    c: u8,

    command: u8,
    featurel: u8

    // DWORD 1
    lba0: u8,
    lba1: u8,
    lba2: u8,
    device: u8,

    // DWORD 2
    lba3: u8,
    lba4: u8,
    lba5: u8,
    featureh: u8,

    // DWORD 3
    countl: u8,
    counth: u8,
    icc: u8,
    control: u8,

    // DWORD 4
    rsv1: u8,
}

pub struct FIS_REG_D2H {
    // DWORD 0
    fis_type: u8,

    pmport: u8,
    rsv0: u8,
    i: u8,
    rsv1: u8,

    status: u8,
    error: u8,

    // DWORD 1
    lba0: u8,
    lba1: u8,
    lba2: u8,
    device: u8,

    // DWORD 2
    lba3: u8,
    lba4: u8,
    lba5: u8,
    rsv2: u8,

    // DWORD 3
    countl: u8,
    counth: u8,
    rsv3: u8,

    // DWORD 4
    rsv4: u8,
}

// TODO: Document
pub struct FIS_DATA {
    // DWORD 0
    fis_type: u8,

    pmport: u8,
    rsv0: u8,

    rsv1: u8,

    // DWORD1 ~ N
    data: u32,
}

// TODO: Document
pub struct FIS_PIO_SETUP {
    // DWORD 0
    fis_type: u8,

    pmport: u8,
    rsv0: u8,
    d: u8,
    i: u8,
    rsv1: u8,

    status: u8,
    error: u8,

    // DWORD 1
    lba0: u8,
    lba1: u8,
    lba2: u8,
    device: u8,

    // DWORD 2
    lba3: u8,
    lba4: u8,
    lba5: u8,
    rsv2: u8,

    // DWORD 3
    countl: u8,
    counth: u8,
    rsv3: u8,
    e_status: u8,

    // DWORD 4
    tc: u16,
    rsv4: u8,
}

// TODO: Document
pub struct FIS_DMA_SETUP {
    // DWORD 0
    fis_type: u8,

    pmport: u8,
    rsv0: u8,
    d: u8,
    i: u8,
    a: u8,

    rsved: u8,

    // DWORD 1&2
    
    DMAbufferID: u64,

    // DWORD 3
    rsvd: u32,

    // DWORD 4
    DMAbufOffset: u32,

    // DWORD 5
    TransferCount: u32,

    // DWORD 6
    resvd: u32,
}

// TODO: Document
pub struct HBA_MEM {
    // 0x00 - 0x2B, Generic Host Control
    cap: u32,
    ghc: u32,
    is: u32,
    pi: u32,
    vs: u32,
    ccc_ctl: u32,
    ccc_pts: u32,
    em_loc: u32,
    em_ctl: u32,
    cap2: u32,
    bohc: u32,

    // 0x2C - 0x9F, Reserved
    rsv: u8,

    // 0xA0 - 0xFF, Vendor specific registers
    vendor: u8,

    // 0x100 - 0x10FF, Port control registers
    ports: HBA_PORT,
}

pub struct HBA_PORT {
    clb: u32,
    clbu: u32,
    fb: u32,
    fbu: u32,
    is: u32,
    ie: u32,
    cmd: u32,
    rsv0: u32,
    tfd: u32,
    sig: u32,
    ssts: u32,
    sctl: u32,
    serr: u32,
    sact: u32,
    ci: u32,
    sntf: u32,
    fbs: u32,
    rsv1: u32,
    vendor: u32,
}

pub struct HBA_FIS {
    // 0x00
    dsfis: FIS_DMA_SETUP,
    pad0: u8,

    // 0x20
    psfis: FIS_PIO_SETUP,
    pad1: u8,
    
    // 0x40
    rfis: FIS_REG_D2H,
    pad2: u8,

    // 0x58
    sdbfis: FIS_DEV_BITS,

    // 0x60
    ufis: u8,

    // 0xA0
    rsv: u8,
}

// TODO: Document
pub struct HBA_CMD_HEADER {
    // DWORD 0
    cfl: u8,
    a: u8,
    w: u8,
    p: u8,

    r: u8,
    b: u8,
    c: u8,
    rsv0: u8,
    pmp: u8,

    prdtl: u16,

    // DWORD 1
    prdbc: u32,

    // DWORD 2, 3
    ctba: u32,
    ctbau: u32,

    // DWORD 4-7
    rsv1: u8,
}

pub struct HBA_CMD_TBL {
    // 0x00
    cfis: u8,

    // 0x40
    acmd: u8,

    // 0x50
    rsv: u8,

    // 0x80
    prdt_entry: HBA_PRDT_ENTRY,
}

pub struct HBA_PRDT_ENTRY {
    dba: u32,
    dbau: u32,
    rsv0: u32, 

    // DWORD 3
    dbc: u32,
    rsv1: u32,
    i: u32,
}
