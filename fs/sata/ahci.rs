#![no_std]

mod ext4;
mod memory;

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

pub struct tagFIS_REG_H2D {
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

pub struct tagFIS_REG_D2H {
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
pub struct tagFIS_DATA {
    // DWORD 0
    fis_type: u8,

    pmport: u8,
    rsv0: u8,

    rsv1: u8,

    // DWORD1 ~ N
    data: u32,
}

// TODO: Document
pub struct tagFIS_PIO_SETUP {
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
