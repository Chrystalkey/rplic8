use std::fmt::{Display, Debug};

mod items;

struct Money{
    kreuzer: u64,
}
impl Money {
    fn from_kreuzer(kreuzer: u32) -> Self{ Self{kreuzer: kreuzer as u64}}
    fn from_heller(heller: f32) -> Self{Self{kreuzer: (heller*10.).floor() as u64}}
    fn from_silber(silber: f32) -> Self{Self{kreuzer: (silber*100.).floor() as u64}}
    fn from_dukaten(dukaten: f32) -> Self{Self{kreuzer: (dukaten*1000.).floor() as u64}}
}
impl Display for Money{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kreuzer < 10 {
            write!(f, "{:1} K", self.kreuzer)
        }else if self.kreuzer < 100 {
            write!(f, "{:1} H {:1} K", self.kreuzer / 10, self.kreuzer % 10)
        }else if self.kreuzer < 1000 {
            write!(f, "{:1} S {:1} H {:1} K", self.kreuzer / 100, (self.kreuzer % 100) / 10, self.kreuzer % 10)
        }else{
            write!(f, "{:1} D {:1} S {:1} H {:1} K",  self.kreuzer / 1000, (self.kreuzer % 1000) / 100, (self.kreuzer % 100) / 10, self.kreuzer % 10)
        }
    }
}
impl Debug for Money{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

struct Coins {
    d: u16,
    s: u16,
    h: u16,
    k: u16,
}

impl From<Coins> for Money{
    fn from(value: Coins) -> Self {
        return Self { kreuzer: value.d as u64*1000+value.s as u64*100+value.h as u64*10+value.k as u64 }
    }
}

struct Loot{
    items: Vec<items::LootItem>,
    coins: Coins,
}

struct Schatzi{

}

impl Schatzi{
    fn new(
        area: Area,
        strt: Stratum,
    ) -> Self{
        todo!()
    }
}

impl generate::RPGenerator for Schatzi{
    type Seed = u64;
    fn seed(&mut self, s: Self::Seed) {
        
    }
}

impl Iterator for Schatzi{
    type Item = Loot;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}