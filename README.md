# DnaRank

This is a work-in-progress implementation of a fast data structure for /rank/
queries over a 2-bit DNA alphabet.

The main API looks like this:

``` rust
impl DnaRank {
    /// Take a DNA string over ACGT characters.
    pub fn new(seq: &[u8]) -> Self;
    
    /// Count the number of A, C, G, *and* T characters before the given position.
    pub fn count4(&self, pos: usize) -> [u32; 4];
}
```
