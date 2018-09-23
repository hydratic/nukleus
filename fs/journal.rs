#![no_std]

extern crate ux;

#[macro_use]
mod phone;

mod library;
mod memory;
mod sata;

pub struct Journal {
    date_created: i64,
    max_size: i64,
    status: i8,
    last_cleaned: i64,
}

pub const JOURNAL_SECTOR: i16 = 0;
pub const JOURNAL_STATUS: i8 = 0;

pub fn new_journal(sector: i16, old_journal: i16, restart: i2) {
    loop {
        if BACKUP == 1 {
            make_journal!(sector, old_journal);

            if restart == 1 {
                unsafe {
                    restart!();
                }       }

                check_journal!();

                if PASS == 1 { break }
            } else {
                let msg: String = "Restarts are recommended for journal testing. Are you sure you would like to proceed without one?";
                query!(msg)

                    if RESULT == 1 {
                        test_journal!(sector, old_journal);
                        if PASS == 1 {
                            delete!(old_journal);
                        }
                    }
            }
        } else {
            make_backup!(sector, old_journal);
            test_backup!(sector, old_journal, restart);

            unsafe {
                BACKUP += 1;	
            }
        }

        if BREAK == 1 { break; }
    }
}

pub fn check_journal() {
    let size = journal_size!(JOURNAL_SECTOR);
    if size > 1000000000 {
        unsafe {
            JOURNAL_STATUS += 3;
        }

        let status: &str = "needs cleaned.";
        pub const JOURNAL_STATUS: &str = status;
    }

    if size == 0 {
        panic!("Journal not detected.");
    }

    if size < 1000000000 {
        if JOURNAL_STATUS > 1 {
            unsafe {
                JOURNAL_STATUS -= 1;
            }

            let status: &str = "needs cleaned.";
            pub const JOURNAL_STATUS: &str = status;
        } else { break; }
    }
}
