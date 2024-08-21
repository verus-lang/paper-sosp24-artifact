#![allow(unused_imports)]
use crate::verif_pt::definitions_t::{MapResult, PageTableEntryExec, UnmapResult};
use crate::verif_pt::impl_u::l2_impl;
use crate::verif_pt::mem_t as mem;
use crate::verif_pt::pervasive::*;

pub trait PTImpl {
    fn implspec_map_frame(
        &self,
        memory: mem::PageTableMemory,
        base: usize,
        pte: PageTableEntryExec,
    ) -> (MapResult, mem::PageTableMemory);
    fn implspec_unmap(
        &self,
        memory: mem::PageTableMemory,
        base: usize,
    ) -> (UnmapResult, mem::PageTableMemory);
}
