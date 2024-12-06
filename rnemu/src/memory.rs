use core::panic;


use crate::common::{Paddr, Word};

const MBASE: usize = 0x80000000;
const MSIZE: usize = 0x8000000;
const PMEM_LEFT: usize = MBASE;
const PMEM_RIGHT: usize = MBASE + MSIZE - 1;
const PC_RESET_OFFSET: usize = 0x0;
const RESET_VECTOR: usize = PMEM_LEFT + PC_RESET_OFFSET;

static mut PMEM: [u8; MSIZE] = [0; MSIZE];

// pub struct MemoryBank {
//     data: Vec<u8>,
// }

fn host_read(addr: usize, len: usize) -> Word {
    unsafe {
        match len {
            1 => {
                let addr = addr as *mut u8;
                (*addr) as Word
            }
            2 => {
                let addr = addr as *mut u16;
                (*addr) as Word
            }
            4 => {
                let addr = addr as *mut u32;
                (*addr) as Word
            }
            _ => panic!(),
        }
    }
}

// convert the guest physical address in the guest program to host virtual address in NEMU
fn host_write(addr: usize, len: usize, data: Word) {
    unsafe {
        match len {
            1 => {
                let addr = addr as *mut u8;
                *addr = data as u8;
            }
            2 => {
                let addr = addr as *mut u16;
                *addr = data as u16;
            }
            4 => {
                let addr = addr as *mut u32;
                *addr = data;
            }
            _ => {
                panic!()
            }
        }
    }
}

pub fn guest_to_host(paddr: Paddr) -> usize {
    unsafe { return PMEM.as_ptr() as usize + paddr as usize - MBASE }
}
// convert the host virtual address in NEMU to guest physical address in the guest program
pub fn host_to_guest(haddr: *const u8) -> Paddr {
    unsafe { (haddr as usize - PMEM.as_ptr() as usize + MBASE as usize) as Paddr }
}

fn pmem_read(addr: Paddr, len: usize) -> Word {
    host_read(guest_to_host(addr), len)
}

fn pmem_write(addr: Paddr, len: usize, data: Word) {
    host_write(guest_to_host(addr), len, data);
}

fn out_of_bound(addr: Paddr) {
    //   panic("address =  {}  is out of bound of pmem [ {} ,  {} ] at pc =  {}" ,
    //       addr, PMEM_LEFT, PMEM_RIGHT, 0x0000);
}

fn init_mem() {
    // info!("physical memory area [" {} ", " {} "]", PMEM_LEFT, PMEM_RIGHT);
}

fn paddr_read(addr: Paddr, len: usize) {
    //   if (likely(in_pmem(addr))) return pmem_read(addr, len);
    //   IFDEF(CONFIG_DEVICE, return mmio_read(addr, len));
    //   out_of_bound(addr);
    //   return 0;
}

fn in_pmem(addr: Paddr) -> bool {
    addr as usize - MBASE < MSIZE
}

fn paddr_write(addr: Paddr, len: usize, data: Word) {
    if in_pmem(addr) {
        pmem_write(addr, len, data);
        return;
    }
    //   IFDEF(CONFIG_DEVICE, mmio_write(addr, len, data); return);
    out_of_bound(addr);
}
