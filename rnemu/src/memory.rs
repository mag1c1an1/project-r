use core::panic;
use std::io::Write;

use crate::common::{Paddr, Vaddr, Word};

const MBASE: usize = 0x8000_0000;
const MSIZE: usize = 0x800_0000;
const PMEM_LEFT: usize = MBASE;
const PMEM_RIGHT: usize = MBASE + MSIZE - 1;
const PC_RESET_OFFSET: usize = 0x0;
pub const RESET_VECTOR: usize = PMEM_LEFT + PC_RESET_OFFSET;

pub struct MemoryBank {
    base: usize,
    data: Vec<u8>,
}

impl MemoryBank {
    pub fn inst_fetch(&self, pc: &mut Vaddr, len: usize) -> Word {
        let start = (*pc - self.base as u32) as usize;
        let ret = Word::from_le_bytes((&self.data[start..start + len]).try_into().unwrap());
        *pc += len as u32;
        ret
    }

    pub fn new(img: &[u8]) -> Self {
        let mut data: Vec<u8> = vec![0; MSIZE];
        let start = RESET_VECTOR - MBASE;
        &mut data[start..start + img.len()].copy_from_slice(img);
        Self { base: MBASE, data }
    }
}

// fn host_read(addr: usize, len: usize) -> Word {
//     unsafe {
//         match len {
//             1 => {
//                 let addr = addr as *mut u8;
//                 (*addr) as Word
//             }
//             2 => {
//                 let addr = addr as *mut u16;
//                 (*addr) as Word
//             }
//             4 => {
//                 let addr = addr as *mut u32;
//                 (*addr) as Word
//             }
//             _ => panic!(),
//         }
//     }
// }

// // convert the guest physical address in the guest program to host virtual address in NEMU
// fn host_write(addr: usize, len: usize, data: Word) {
//     unsafe {
//         match len {
//             1 => {
//                 let addr = addr as *mut u8;
//                 *addr = data as u8;
//             }
//             2 => {
//                 let addr = addr as *mut u16;
//                 *addr = data as u16;
//             }
//             4 => {
//                 let addr = addr as *mut u32;
//                 *addr = data;
//             }
//             _ => {
//                 panic!()
//             }
//         }
//     }
// }

// pub fn guest_to_host(paddr: Paddr) -> usize {
//     unsafe { return PMEM.as_ptr() as usize + paddr as usize - MBASE }
// }
// // convert the host virtual address in NEMU to guest physical address in the guest program
// pub fn host_to_guest(haddr: *const u8) -> Paddr {
//     unsafe { (haddr as usize - PMEM.as_ptr() as usize + MBASE as usize) as Paddr }
// }

// fn pmem_read(addr: Paddr, len: usize) -> Word {
//     host_read(guest_to_host(addr), len)
// }

// fn pmem_write(addr: Paddr, len: usize, data: Word) {
//     host_write(guest_to_host(addr), len, data);
// }

// fn out_of_bound(addr: Paddr) {
//     //   panic("address =  {}  is out of bound of pmem [ {} ,  {} ] at pc =  {}" ,
//     //       addr, PMEM_LEFT, PMEM_RIGHT, 0x0000);
// }

// fn init_mem() {
//     // info!("physical memory area [" {} ", " {} "]", PMEM_LEFT, PMEM_RIGHT);
// }

// fn paddr_read(addr: Paddr, len: usize) {
//     //   if (likely(in_pmem(addr))) return pmem_read(addr, len);
//     //   IFDEF(CONFIG_DEVICE, return mmio_read(addr, len));
//     //   out_of_bound(addr);
//     //   return 0;
// }

// fn in_pmem(addr: Paddr) -> bool {
//     addr as usize - MBASE < MSIZE
// }

// fn paddr_write(addr: Paddr, len: usize, data: Word) {
//     if in_pmem(addr) {
//         pmem_write(addr, len, data);
//         return;
//     }
//     //   IFDEF(CONFIG_DEVICE, mmio_write(addr, len, data); return);
//     out_of_bound(addr);
// }

#[derive(Default)]
pub struct Memory {
    /// Memory content
    data: Vec<u64>,
}

impl Memory {
    pub fn read_byte(&self, address: u64) -> u8 {
        todo!()
    }
    pub fn read_halfword(&self, address: u64) -> u16 {
        todo!()
    }
    pub fn read_word(&self, address: u64) -> u32 {
        todo!()
    }
    pub fn read_doubleworld(&self, address: u64) -> u64 {
        todo!()
    }

    /// Reads multiple bytes from memory.
    ///
    /// # Arguments
    /// * `address`
    /// * `width` up to eight
    pub fn read_bytes(&self, address: u64, width: u64) -> u64 {
        let mut data = 0 as u64;
        for i in 0..width {
            data |= (self.read_byte(address.wrapping_add(i)) as u64) << (i * 8);
        }
        data
    }

    /// Writes a byte to memory.
    ///
    /// # Arguments
    /// * `address`
    /// * `value`
    pub fn write_byte(&mut self, address: u64, value: u8) {
        let index = (address >> 3) as usize;
        let pos = ((address % 8) as u64) * 8;
        self.data[index] = (self.data[index] & !(0xff << pos)) | ((value as u64) << pos);
    }

    /// Writes two bytes to memory.
    ///
    /// # Arguments
    /// * `address`
    /// * `value`
    pub fn write_halfword(&mut self, address: u64, value: u16) {
        if (address % 2) == 0 {
            let index = (address >> 3) as usize;
            let pos = ((address % 8) as u64) * 8;
            self.data[index] = (self.data[index] & !(0xffff << pos)) | ((value as u64) << pos);
        } else {
            self.write_bytes(address, value as u64, 2);
        }
    }

    /// Writes four bytes to memory.
    ///
    /// # Arguments
    /// * `address`
    /// * `value`
    pub fn write_word(&mut self, address: u64, value: u32) {
        if (address % 4) == 0 {
            let index = (address >> 3) as usize;
            let pos = ((address % 8) as u64) * 8;
            self.data[index] = (self.data[index] & !(0xffffffff << pos)) | ((value as u64) << pos);
        } else {
            self.write_bytes(address, value as u64, 4);
        }
    }

    /// Writes eight bytes to memory.
    ///
    /// # Arguments
    /// * `address`
    /// * `value`
    pub fn write_doubleword(&mut self, address: u64, value: u64) {
        if (address % 8) == 0 {
            let index = (address >> 3) as usize;
            self.data[index] = value;
        } else if (address % 4) == 0 {
            self.write_word(address, (value & 0xffffffff) as u32);
            self.write_word(address.wrapping_add(4), (value >> 32) as u32);
        } else {
            self.write_bytes(address, value, 8);
        }
    }

    /// Write multiple bytes to memory.
    ///
    /// # Arguments
    /// * `address`
    /// * `value`
    /// * `width` up to eight
    pub fn write_bytes(&mut self, address: u64, value: u64, width: u64) {
        for i in 0..width {
            self.write_byte(address.wrapping_add(i), (value >> (i * 8)) as u8);
        }
    }

    /// Check if the address is valid memory address
    ///
    /// # Arguments
    /// * `address`
    pub fn validate_address(&self, address: u64) -> bool {
        return (address as usize) < self.data.len();
    }
}
