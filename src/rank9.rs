use mem_dbg::MemSize;
use sux::{bits::BitVec, prelude::Rank9, traits::Rank};

use crate::ranker::RankerT;

impl RankerT for Rank9<BitVec<Vec<usize>>> {
    fn new(seq: &[u8]) -> Self {
        let mut bits = BitVec::with_capacity(seq.len() * 2);
        for x in seq {
            bits.push(((x >> 1) & 1) != 0);
            bits.push(((x >> 2) & 1) != 0);
        }
        Rank9::new(bits)
    }

    fn prefetch(&self, _pos: usize) {}

    fn size(&self) -> usize {
        self.mem_size(Default::default())
    }

    #[inline(always)]
    fn count(&self, pos: usize) -> crate::Ranks {
        [self.rank(pos) as u32, 0, 0, 0]
    }

    #[inline(always)]
    fn count1(&self, pos: usize, _c: u8) -> u32 {
        self.rank(pos) as u32
    }
}
