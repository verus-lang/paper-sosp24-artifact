extern crate alloc;
pub mod nros_vmem;
pub mod verif_pt;
use verif_pt::mem_t as mem;
use std::time::Instant;

use x86::bits64::paging;

fn benchmark_nros_vmem() {
    let mut vmem = nros_vmem::VSpace {
        pml4: Box::pin(
            [paging::PML4Entry::new(paging::PAddr::from(0u64), paging::PML4Flags::empty()); paging::PAGE_SIZE_ENTRIES],
        ),
        allocs: Vec::with_capacity(10240),
    };
    for i in 0..1_000_000 {
        vmem.map_generic(paging::VAddr::from_usize(i * 4096), (paging::PAddr::from(0u64), 4096), nros_vmem::MapAction::ReadWriteUser);
        // println!("{:x?}", pt.resolve(i * 4096));
    }
    let time_before = Instant::now();
    for i in 1_000_000..101_000_000 {
        vmem.map_generic(
            paging::VAddr::from_usize(i * 4096),
            (paging::PAddr::from(0u64), 4096),
            nros_vmem::MapAction::ReadWriteUser);
        // println!("{:x?}", pt.resolve(i * 4096));
    }
    let time = time_before.elapsed();
    println!("Time NrOS Mapping: {} ns", time.as_nanos() as f64 / 100_000_000.0);

    // unmap

    for i in 0..1_000_000 {
        vmem.unmap(paging::VAddr::from_usize(i * 4096)).unwrap();
        // println!("{:x?}", pt.resolve(i * 4096));
    }
    let time_before = Instant::now();
    for i in 1_000_000..101_000_000 {
        vmem.unmap(paging::VAddr::from_usize(i * 4096)).unwrap();
        // println!("{:x?}", pt.resolve(i * 4096));
    }
    let time = time_before.elapsed();
    println!("Time NrOS Unmapping: {} ns", time.as_nanos() as f64 / 100_000_000.0);
}

fn alloc_page() -> usize {
    let ptr: *mut u8 = unsafe {
        alloc::alloc::alloc(core::alloc::Layout::from_size_align_unchecked(
            4096,
            4096,
        ))
    };
    ptr as usize
}

fn benchmark_verif_noreclaim() {
    let root_dir1: *mut u8 = unsafe {
        alloc::alloc::alloc(core::alloc::Layout::from_size_align_unchecked(
            4096,
            4096,
        ))
    };
    let mem = mem::PageTableMemory {
        ptr: 0 as *mut u64,
        pml4: root_dir1 as usize,
        pt_allocator: Box::new(alloc_page),
    };
    let mut pt = verif_pt::impl_u::l2_impl::PageTable {
        memory: mem,
        arch: verif_pt::definitions_t::x86_arch_exec(),
        ghost_pt: (),
    };
    let pte = verif_pt::definitions_t::PageTableEntryExec {
        flags: verif_pt::definitions_t::Flags {
            disable_execute: false,
            is_writable: true,
            is_supervisor: false,
        },
        frame: verif_pt::definitions_t::MemRegionExec {
            base: 0,
            size: 4096,
        },
    };
    for i in 0..101_000_000 {
        pt.map_frame(i * 4096, pte.clone());
    }

    for i in 0..1_000_000 {
        pt.unmap_noreclaim(i * 4096);
    }
     
    let time_before = Instant::now();
    for i in 1_000_000..101_000_000 {
        pt.unmap_noreclaim(i * 4096);
        // match pt.unmap_noreclaim(i * 4096) {
        //     verif_pt::definitions_t::UnmapResult::ErrNoSuchMapping => panic!(),
        //     _ => {},
        // }
    }
    let time = time_before.elapsed();
    println!("Time Verified PT Unmapping (no reclaim): {} ns", time.as_nanos() as f64 / 100_000_000.0);
}

fn benchmark_verif() {
    let root_dir1: *mut u8 = unsafe {
        alloc::alloc::alloc(core::alloc::Layout::from_size_align_unchecked(
            4096,
            4096,
        ))
    };
    let mem = mem::PageTableMemory {
        ptr: 0 as *mut u64,
        pml4: root_dir1 as usize,
        pt_allocator: Box::new(alloc_page),
    };
    let mut pt = verif_pt::impl_u::l2_impl::PageTable {
        memory: mem,
        arch: verif_pt::definitions_t::x86_arch_exec(),
        ghost_pt: (),
    };
    let pte = verif_pt::definitions_t::PageTableEntryExec {
        flags: verif_pt::definitions_t::Flags {
            disable_execute: false,
            is_writable: true,
            is_supervisor: false,
        },
        frame: verif_pt::definitions_t::MemRegionExec {
            base: 0,
            size: 4096,
        },
    };
    let mut _x: u32 = 0;
    // let pre = unsafe { core::arch::x86_64::__rdtscp(&mut _x as *mut u32) };
    // println!("x");
    for i in 0..1_000_000 {
        pt.map_frame(i * 4096, pte.clone());
    }
    let time_before = Instant::now();
    for i in 1_000_000..101_000_000 {
        pt.map_frame(i * 4096, pte.clone());
        // match pt.map_frame(i * 4096, pte.clone()) {
        //     verif_pt::definitions_t::MapResult::ErrOverlap => panic!(),
        //     _ => {},
        // }
        // println!("{:x?}", pt.resolve(i * 4096));
    }
    let time = time_before.elapsed();
    println!("Time Verified PT Mapping: {} ns", time.as_nanos() as f64 / 100_000_000.0);
    // let post = unsafe { core::arch::x86_64::__rdtscp(&mut _x as *mut u32) };
    // let cycles = post - pre;
    // println!("Cycles: {}", cycles);

    for i in 0..1_000_000 {
        pt.unmap(i * 4096);
    }
     
    let time_before = Instant::now();
    for i in 1_000_000..101_000_000 {
        match pt.unmap(i * 4096) {
            verif_pt::definitions_t::UnmapResult::ErrNoSuchMapping => panic!(),
            _ => {},
        }
        // println!("{:x?}", pt.resolve(i * 4096));
    }
    let time = time_before.elapsed();
    println!("Time Verified PT Unmapping: {} ns", time.as_nanos() as f64 / 100_000_000.0);
}

fn main() {
    benchmark_verif();
    benchmark_verif_noreclaim();
    benchmark_nros_vmem();
}
