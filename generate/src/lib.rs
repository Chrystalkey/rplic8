pub trait RPGenerator: Iterator + Sync + Send {
    type Seed;
    fn seed(&mut self, s: Self::Seed);
}