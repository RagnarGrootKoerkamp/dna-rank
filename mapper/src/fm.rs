use quadrank::{QuadRank, ranker::RankerT};

pub struct FM {
    rank: QuadRank,
    occ: Vec<u32>,
}

impl FM {
    pub fn new(bwt: &[u8]) -> Self {
        let rank = QuadRank::new(bwt);
        let mut occ = vec![0; 4];
        for &c in bwt {
            occ[c as usize] += 1;
        }
        for i in 1..4 {
            occ[i] += occ[i - 1];
        }
        Self { rank, occ }
    }

    pub fn query(&self, text: &[u8]) -> usize {
        let mut s: u32 = 0;
        let mut t: u32 = text.len().try_into().unwrap();
        for &c in text {
            s = self.occ[c as usize] + self.rank.count(s as usize)[c as usize];
            t = self.occ[c as usize] + self.rank.count(t as usize)[c as usize];
            if s == t {
                return 0;
            }
        }
        (t - s) as usize
    }
}
